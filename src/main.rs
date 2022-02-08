mod ace_bindings;
mod app;
mod koto_wrapper;

use {
    crate::{ace_bindings::get_ace, app::App, koto_wrapper::KotoMessageQueue},
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_utils::document,
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::Element,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    static APP: RefCell<App> = RefCell::new(App::new());
    static KOTO_MESSAGE_QUEUE: KotoMessageQueue = Rc::new(RefCell::new(VecDeque::new()));
}

fn main() {
    set_panic_hook();
    register_koto_editor_mode();
    setup_editor();
    setup_app();
}

#[wasm_bindgen(module = "/src/koto-highlight-rules.js")]
extern "C" {
    fn register_koto_editor_mode();
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
    APP.with(|app| app.borrow_mut().compile_script_and_call_setup());
}

fn get_element_by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Failed to get div with id '{id}'"))
}
