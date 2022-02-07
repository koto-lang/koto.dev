#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use {
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_events::EventListener,
    gloo_render::{request_animation_frame, AnimationFrame},
    gloo_utils::{document, window},
    instant::Instant,
    js_sys::Function,
    koto::{
        runtime::{
            unexpected_type_error_with_slice, KotoFile, KotoRead, KotoWrite, RuntimeError, Value,
            ValueMap,
        },
        Koto, KotoError, KotoSettings,
    },
    std::{cell::RefCell, collections::VecDeque, fmt, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement},
};

#[wasm_bindgen(module = "/src/koto-highlight-rules.js")]
extern "C" {
    fn register_koto_editor_mode();
}

#[wasm_bindgen]
extern "C" {
    type Ace;
    type AceEditor;
    type AceSession;

    #[wasm_bindgen(method)]
    fn edit(this: &Ace, id: &str) -> AceEditor;

    #[wasm_bindgen(method, js_name = getSession)]
    fn get_session(this: &AceEditor) -> AceSession;

    #[wasm_bindgen(method, js_name = setTheme)]
    fn set_theme(this: &AceEditor, theme: &str);

    #[wasm_bindgen(method, js_name = setShowPrintMargin)]
    fn set_show_print_margin(this: &AceEditor, value: bool);

    #[wasm_bindgen(method, js_name = setMode)]
    fn set_mode(this: &AceSession, mode: &str);

    #[wasm_bindgen(method, js_name = getValue)]
    fn get_value(this: &AceSession) -> String;

    #[wasm_bindgen(method)]
    fn on(this: &AceSession, event_name: &str, callback: &Function);
}

#[wasm_bindgen(inline_js = "export function get_ace() { return ace; }")]
extern "C" {
    fn get_ace() -> Ace;
}

fn main() {
    set_panic_hook();
    register_koto_editor_mode();
    setup_editor();
    setup_app();
}

fn get_element_by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Failed to get div with id '{id}'"))
}

fn setup_editor() {
    let editor_id = "editor";
    let editor_div = get_element_by_id("editor");
    editor_div.set_inner_html(include_str!("default.koto"));

    let ace = get_ace();
    let editor = ace.edit(editor_id);
    editor.set_theme("ace/theme/solarized_dark");
    editor.set_show_print_margin(false);

    let session = editor.get_session();
    session.set_mode("ace/mode/koto");

    let on_change = Closure::wrap(
        Box::new(|| APP.with(move |app| app.borrow_mut().run_script())) as Box<dyn Fn()>,
    );
    session.on("change", on_change.as_ref().unchecked_ref());
    on_change.forget();
}

fn setup_app() {
    APP.with(|app| app.borrow_mut().run_script());
}

type KotoMessageQueue = Rc<RefCell<VecDeque<KotoMsg>>>;

thread_local! {
    static APP: RefCell<App> = RefCell::new(App::new());
    static KOTO_MESSAGE_QUEUE: KotoMessageQueue = Rc::new(RefCell::new(VecDeque::new()));
}

struct App {
    koto: Koto,
    compiler_output: Element,
    script_output: Element,
    canvas: HtmlCanvasElement,
    canvas_context: CanvasRenderingContext2d,
    output_buffer: String,
    message_queue: KotoMessageQueue,
    _window_resize_listener: EventListener,
    animation_frame: AnimationFrame,
}

impl App {
    fn new() -> Self {
        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_stdout(OutputCapture {})
                .with_stderr(OutputCapture {}),
        );

        koto.prelude().add_map("canvas", make_canvas_module());

        let canvas = get_element_by_id("koto-canvas");
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .expect("koto-canvas is the wrong element type");

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let canvas_context = canvas
            .get_context("2d")
            .expect("Error while getting canvas context")
            .expect("Missing canvas context")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Error while casting canvas context");

        canvas_context.set_fill_style(&JsValue::from("#999999"));
        canvas_context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        let window_resize_listener = EventListener::new(&window(), "resize", |_| {
            APP.with(|app| app.borrow_mut().on_window_resize());
        });

        Self {
            koto,
            compiler_output: get_element_by_id("compiler-output"),
            script_output: get_element_by_id("script-output"),
            canvas,
            canvas_context,
            output_buffer: String::with_capacity(128),
            message_queue: KOTO_MESSAGE_QUEUE.with(|q| q.clone()),
            _window_resize_listener: window_resize_listener,
            animation_frame: request_animation_frame(Self::on_animation_frame_handler),
        }
    }

    fn run_script(&mut self) {
        let input = get_ace().edit("editor").get_session().get_value();
        let now = Instant::now();

        match self.koto.compile(&input).and_then(|_| self.koto.run()) {
            Ok(_) => {
                let elapsed_ms = now.elapsed().as_millis();
                let success_string = format!("Success ({elapsed_ms}ms)");
                self.compiler_output.set_inner_html(&success_string);
                self.script_output.set_inner_html("");
                self.process_koto_messages();
            }
            Err(error) => {
                let error_string = match error {
                    KotoError::RuntimeError(_) => format!("Runtime error: {error}"),
                    _ => format!("Error: {error}"),
                };
                self.compiler_output.set_inner_html(&error_string);
            }
        }
    }

    fn on_animation_frame_handler(time_delta: f64) {
        APP.with(|app| app.borrow_mut().on_animation_frame(time_delta));
    }

    fn on_animation_frame(&mut self, _time_delta: f64) {
        self.run_script();
        self.animation_frame = request_animation_frame(Self::on_animation_frame_handler);
    }

    fn process_koto_messages(&mut self) {
        for message in self.message_queue.borrow_mut().drain(..) {
            match message {
                KotoMsg::Print(s) => {
                    self.output_buffer.push_str(&s);
                }
                KotoMsg::Clear => {
                    self.canvas_context.clear_rect(
                        0.0,
                        0.0,
                        self.canvas.width() as f64,
                        self.canvas.height() as f64,
                    );
                }
                KotoMsg::BeginPath => {
                    self.canvas_context.begin_path();
                }
                KotoMsg::MoveTo { x, y } => {
                    self.canvas_context.move_to(x, y);
                }
                KotoMsg::Arc {
                    x,
                    y,
                    radius,
                    start_angle,
                    end_angle,
                    counter_clockwise,
                } => {
                    self.canvas_context
                        .arc_with_anticlockwise(
                            x,
                            y,
                            radius,
                            start_angle,
                            end_angle,
                            counter_clockwise,
                        )
                        .expect("Failed to draw arc");
                }
                KotoMsg::Fill => {
                    self.canvas_context.fill();
                }
                KotoMsg::Stroke => {
                    self.canvas_context.stroke();
                }
            }
        }

        if !self.output_buffer.is_empty() {
            self.script_output
                .append_with_str_1(&self.output_buffer)
                .expect("Failed to append to script output");
            self.output_buffer.clear();
        }
    }

    fn on_window_resize(&mut self) {
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);
    }
}

enum KotoMsg {
    Print(String),
    Clear,
    BeginPath,
    MoveTo {
        x: f64,
        y: f64,
    },
    Arc {
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counter_clockwise: bool,
    },
    Fill,
    Stroke,
}

fn send_koto_message(message: KotoMsg) {
    KOTO_MESSAGE_QUEUE.with(|q| q.borrow_mut().push_back(message));
}

fn make_canvas_module() -> ValueMap {
    use Value::*;

    let result = ValueMap::default();

    result.add_fn("begin_path", |_, _| {
        send_koto_message(KotoMsg::BeginPath);
        Ok(Empty)
    });

    result.add_fn("clear", |_, _| {
        send_koto_message(KotoMsg::Clear);
        Ok(Empty)
    });

    result.add_fn("fill", |_, _| {
        send_koto_message(KotoMsg::Fill);
        Ok(Empty)
    });

    result.add_fn("move_to", |vm, args| {
        let (x, y) = match vm.get_args(args) {
            [Number(x), Number(y)] => (x.into(), y.into()),
            [Num2(n)] => (n[0], n[1]),
            unexpected => {
                return unexpected_type_error_with_slice(
                    "canvas.move_to",
                    "two Numbers or a Num2",
                    unexpected,
                )
            }
        };
        send_koto_message(KotoMsg::MoveTo { x, y });
        Ok(Empty)
    });

    result.add_fn("arc", |vm, args| {
        let (x, y, radius, start_angle, end_angle, counter_clockwise) = match vm.get_args(args) {
            [Num2(n), Number(radius), Number(start_angle), Number(end_angle)] => {
                (n[0], n[1], radius.into(), start_angle.into(), end_angle.into(), false)
            },
            [Num2(n), Number(radius), Number(start_angle), Number(end_angle), Bool(counter_clockwise)] => {
                (n[0], n[1], radius.into(), start_angle.into(), end_angle.into(), *counter_clockwise)
            }
            [Number(x), Number(y), Number(radius), Number(start_angle), Number(end_angle)] => {
                (x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), false)
            }
            [Number(x), Number(y), Number(radius), Number(start_angle), Number(end_angle), Bool(counter_clockwise)] => {
                (x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), *counter_clockwise)
            }
            unexpected => {
                return unexpected_type_error_with_slice(
                    "canvas.arc",
                    "x & y (as Numbers or a Num2), radius, start_angle, end_angle, counter_clockwise (optional)",
                    unexpected,
                )
            }
        };
        send_koto_message(KotoMsg::Arc {
            x,
            y,
            radius,
            start_angle,
            end_angle,
            counter_clockwise,
        });
        Ok(Empty)
    });

    result.add_fn("stroke", |_, _| {
        send_koto_message(KotoMsg::Stroke);
        Ok(Empty)
    });

    result
}

// Captures output from Koto in a String
#[derive(Debug)]
struct OutputCapture {}

impl KotoFile for OutputCapture {}
impl KotoRead for OutputCapture {}

impl KotoWrite for OutputCapture {
    fn write(&self, bytes: &[u8]) -> Result<(), RuntimeError> {
        let bytes_str = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string().into()),
        };
        KOTO_MESSAGE_QUEUE.with(|q| {
            q.borrow_mut()
                .push_back(KotoMsg::Print(bytes_str.to_string()))
        });
        Ok(())
    }

    fn write_line(&self, output: &str) -> Result<(), RuntimeError> {
        send_koto_message(KotoMsg::Print(format!("{output}\n")));
        Ok(())
    }

    fn flush(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

impl fmt::Display for OutputCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("_stdout_")
    }
}
