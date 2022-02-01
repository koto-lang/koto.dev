#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use {
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_utils::document,
    js_sys::Function,
    koto::{
        runtime::{KotoFile, KotoRead, KotoWrite, RuntimeError},
        Koto, KotoError, KotoSettings,
    },
    std::{cell::RefCell, fmt, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::Element,
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

thread_local! {
    static APP: Box<RefCell<App>> = Box::new(RefCell::new(App::new()));
}

struct App {
    koto: KotoWrapper,
}

impl App {
    fn new() -> Self {
        Self {
            koto: KotoWrapper::new(),
        }
    }

    fn run_script(&mut self) {
        let input = get_ace().edit("editor").get_session().get_value();
        match self.koto.compile_and_run(&input) {
            Ok(output) => {
                get_element_by_id("script-output").set_inner_html(&output);
                get_element_by_id("compiler-output").set_inner_html("Success");
            }
            Err(error) => {
                let error_string = match error {
                    KotoError::RuntimeError(_) => format!("Runtime error: {error}"),
                    _ => format!("Error: {error}"),
                };
                get_element_by_id("compiler-output").set_inner_html(&error_string);
            }
        }
    }
}

struct KotoWrapper {
    koto: Koto,
    output: Rc<RefCell<String>>,
}

impl KotoWrapper {
    fn new() -> Self {
        let output = Rc::new(RefCell::new(String::new()));

        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_stdout(OutputCapture {
                    output: output.clone(),
                })
                .with_stderr(OutputCapture {
                    output: output.clone(),
                }),
        );

        Self { koto, output }
    }

    fn compile_and_run(&mut self, input: &str) -> Result<String, KotoError> {
        self.koto.compile(input)?;
        self.koto.run()?;
        let output = std::mem::take::<String>(&mut self.output.borrow_mut());
        Ok(output)
    }
}

// Captures output from Koto in a String
#[derive(Debug)]
struct OutputCapture {
    output: Rc<RefCell<String>>,
}

impl KotoFile for OutputCapture {}
impl KotoRead for OutputCapture {}

impl KotoWrite for OutputCapture {
    fn write(&self, bytes: &[u8]) -> Result<(), RuntimeError> {
        let bytes_str = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string().into()),
        };
        self.output.borrow_mut().push_str(bytes_str);
        Ok(())
    }

    fn write_line(&self, output: &str) -> Result<(), RuntimeError> {
        let mut unlocked = self.output.borrow_mut();
        unlocked.push_str(output);
        unlocked.push('\n');
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
