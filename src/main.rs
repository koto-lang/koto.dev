#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use {
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_events::EventListener,
    gloo_render::AnimationFrame,
    gloo_utils::{document, window},
    // instant::Instant,
    js_sys::Function,
    koto::{
        runtime::{
            unexpected_type_error_with_slice, CallArgs, KotoFile, KotoRead, KotoWrite,
            RuntimeError, Value, ValueMap,
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

    let on_change =
        Closure::wrap(
            Box::new(|| APP.with(move |app| app.borrow_mut().on_script_changed())) as Box<dyn Fn()>,
        );
    session.on("change", on_change.as_ref().unchecked_ref());
    on_change.forget();
}

fn setup_app() {
    APP.with(|app| app.borrow_mut().compile_script(true));
}

type KotoMessageQueue = Rc<RefCell<VecDeque<KotoMsg>>>;

thread_local! {
    static APP: RefCell<App> = RefCell::new(App::new());
    static KOTO_MESSAGE_QUEUE: KotoMessageQueue = Rc::new(RefCell::new(VecDeque::new()));
}

struct App {
    koto: Koto,
    play_module: ValueMap,
    user_data: Value,
    is_ready: bool,
    last_time: Option<f64>,
    compiler_output: Element,
    script_output: Element,
    canvas: HtmlCanvasElement,
    canvas_context: CanvasRenderingContext2d,
    output_buffer: String,
    message_queue: KotoMessageQueue,
    _window_resize_listener: EventListener,
    animation_frame: Option<AnimationFrame>,
}

impl App {
    fn new() -> Self {
        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_stdout(OutputCapture {})
                .with_stderr(OutputCapture {}),
        );

        let play_module = ValueMap::default();
        koto.prelude().add_map("canvas", make_canvas_module());
        koto.prelude().add_map("play", play_module.clone());

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
            play_module,
            user_data: Value::Empty,
            is_ready: false,
            last_time: None,
            compiler_output: get_element_by_id("compiler-output"),
            script_output: get_element_by_id("script-output"),
            canvas,
            canvas_context,
            output_buffer: String::with_capacity(128),
            message_queue: KOTO_MESSAGE_QUEUE.with(|q| q.clone()),
            _window_resize_listener: window_resize_listener,
            animation_frame: None,
        }
    }

    fn compile_script(&mut self, call_setup: bool) {
        let script = get_ace().edit("editor").get_session().get_value();

        self.is_ready = false;
        self.compiler_output.set_inner_html("");
        self.script_output.set_inner_html("");
        self.animation_frame = None;
        self.last_time = None;
        self.message_queue.borrow_mut().clear();

        {
            let mut play_module = self.play_module.data_mut();
            play_module.remove_with_string("setup");
            play_module.remove_with_string("on_load");
            play_module.remove_with_string("update");
        }

        self.koto.clear_module_cache();
        if let Err(error) = self.koto.compile(&script) {
            self.log_error(&format!("Error while compiling script: {error}"));
            return;
        }

        if let Err(e) = self.koto.run() {
            self.log_error(&e.to_string());
            return;
        }

        if call_setup {
            self.user_data = match self.play_module.data().get_with_string("setup") {
                Some(f) => match self.koto.run_function(f.clone(), CallArgs::None) {
                    Ok(data) => data,
                    Err(e) => {
                        self.log_error(&e.to_string());
                        return;
                    }
                },
                None => Value::Map(ValueMap::default()),
            };
        }

        if let Err(e) = self.run_play_function("on_load", &[self.user_data.clone()]) {
            self.log_error(&e.to_string());
            return;
        }

        self.is_ready = true;

        if self.play_module.data().get_with_string("update").is_some() {
            self.request_animation_frame();
        }

        self.process_koto_messages();
    }

    fn log_error(&self, error: &str) {
        self.compiler_output.append_with_str_1(error).expect("Failed to log error");
        self.compiler_output.set_scroll_top(self.compiler_output.scroll_height());
    }

    fn run_play_function(
        &mut self,
        function_name: &str,
        args: &[Value],
    ) -> Result<Value, KotoError> {
        match self.play_module.data().get_with_string(function_name) {
            Some(f) => self.koto.run_function(f.clone(), CallArgs::Separate(args)),
            None => Ok(Value::Empty),
        }
    }

    fn run_update(&mut self, time_delta: f64) {
        debug_assert!(self.is_ready);

        match self.run_play_function("update", &[self.user_data.clone(), time_delta.into()]) {
            Ok(_) => {
                self.process_koto_messages();
            }
            Err(e) => {
                self.is_ready = false;
                self.log_error(&e.to_string());
            }
        }
    }

    fn on_script_changed(&mut self) {
        self.compile_script(false);
    }

    fn request_animation_frame(&mut self) {
        self.animation_frame = Some(gloo_render::request_animation_frame(
            Self::on_animation_frame_handler,
        ));
    }

    fn on_animation_frame_handler(time: f64) {
        APP.with(|app| app.borrow_mut().on_animation_frame(time));
    }

    fn on_animation_frame(&mut self, time: f64) {
        let time_delta = if let Some(last_time) = self.last_time {
            (time - last_time) / 1000.0
        } else {
            0.0
        };

        self.last_time = Some(time);

        self.run_update(time_delta);

        if self.is_ready {
            self.request_animation_frame();
        }
    }

    fn process_koto_messages(&mut self) {
        for message in self.message_queue.borrow_mut().drain(..) {
            match message {
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
                KotoMsg::BeginPath => {
                    self.canvas_context.begin_path();
                }
                KotoMsg::Clear => {
                    self.canvas_context.clear_rect(
                        0.0,
                        0.0,
                        self.canvas.width() as f64,
                        self.canvas.height() as f64,
                    );
                }
                KotoMsg::Fill => {
                    self.canvas_context.fill();
                }
                KotoMsg::MoveTo { x, y } => {
                    self.canvas_context.move_to(x, y);
                }
                KotoMsg::Print(s) => {
                    self.output_buffer.push_str(&s);
                }
                KotoMsg::SetFillColor(color) => {
                    let color_rgb = color.as_css_rgb();
                    self.canvas_context
                        .set_fill_style(&JsValue::from(color_rgb))
                }
                KotoMsg::SetLineWidth(width) => self.canvas_context.set_line_width(width),
                KotoMsg::SetStrokeColor(color) => {
                    let color_rgb = color.as_css_rgb();
                    self.canvas_context
                        .set_stroke_style(&JsValue::from(color_rgb))
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
            self.script_output
                .set_scroll_top(self.script_output.scroll_height());
            self.output_buffer.clear();
        }
    }

    fn on_window_resize(&mut self) {
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);
    }
}

enum KotoMsg {
    Arc {
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counter_clockwise: bool,
    },
    BeginPath,
    Clear,
    Fill,
    MoveTo {
        x: f64,
        y: f64,
    },
    Print(String),
    SetFillColor(Color),
    SetLineWidth(f64),
    SetStrokeColor(Color),
    Stroke,
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Color {
    fn as_css_rgb(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
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

    result.add_fn("set_fill_color", |vm, args| {
        let (r, g, b, a) = match vm.get_args(args) {
            [Number(n1), Number(n2), Number(n3)] => (n1.into(), n2.into(), n3.into(), 1.0),
            [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                (n1.into(), n2.into(), n3.into(), n4.into())
            }
            [Num4(color)] => (
                color.0 as f64,
                color.1 as f64,
                color.2 as f64,
                color.3 as f64,
            ),
            unexpected => {
                return unexpected_type_error_with_slice(
                    "canvas.set_fill_color",
                    "3 or 4 Numbers or a Num4",
                    unexpected,
                )
            }
        };
        send_koto_message(KotoMsg::SetFillColor(Color { r, b, g, a }));
        Ok(Empty)
    });

    result.add_fn("set_line_width", |vm, args| {
        let width = match vm.get_args(args) {
            [Number(n)] => n,
            unexpected => {
                return unexpected_type_error_with_slice(
                    "canvas.set_line_width",
                    "a Number",
                    unexpected,
                )
            }
        };
        send_koto_message(KotoMsg::SetLineWidth(width.into()));
        Ok(Empty)
    });

    result.add_fn("set_stroke_color", |vm, args| {
        let (r, g, b, a) = match vm.get_args(args) {
            [Number(n1), Number(n2), Number(n3)] => (n1.into(), n2.into(), n3.into(), 1.0),
            [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                (n1.into(), n2.into(), n3.into(), n4.into())
            }
            [Num4(color)] => (
                color.0 as f64,
                color.1 as f64,
                color.2 as f64,
                color.3 as f64,
            ),
            unexpected => {
                return unexpected_type_error_with_slice(
                    "canvas.set_stroke_color",
                    "3 or 4 Numbers or a Num4",
                    unexpected,
                )
            }
        };
        send_koto_message(KotoMsg::SetStrokeColor(Color { r, b, g, a }));
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
