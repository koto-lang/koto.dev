#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use {
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_events::EventListener,
    gloo_utils::{document, window},
    instant::Instant,
    js_sys::Function,
    koto::{
        runtime::{KotoFile, KotoRead, KotoWrite, RuntimeError},
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
}

impl App {
    fn new() -> Self {
        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_stdout(OutputCapture {})
                .with_stderr(OutputCapture {}),
        );

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

    fn process_koto_messages(&mut self) {
        for message in self.message_queue.borrow_mut().drain(..) {
            match message {
                KotoMsg::Print(s) => {
                    self.output_buffer.push_str(&s);
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
        let context = &mut self.canvas_context;
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);

        context.begin_path();

        use std::f64::consts::PI;
        // Draw the outer circle.
        context.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, PI).unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0).unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0).unwrap();

        context.stroke();
    }
}

enum KotoMsg {
    Print(String),
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
        KOTO_MESSAGE_QUEUE.with(|q| {
            q.borrow_mut()
                .push_back(KotoMsg::Print(format!("{output}\n")))
        });
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
