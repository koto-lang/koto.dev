mod ace_bindings;
mod app;
mod koto_wrapper;

use {
    crate::{ace_bindings::get_ace, app::App, koto_wrapper::KotoMessageQueue},
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_events::EventListener,
    gloo_utils::document,
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{Element, HtmlOptGroupElement, HtmlOptionElement, HtmlSelectElement},
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
    setup_script_menu();
    setup_editor();
    setup_app();
}

#[wasm_bindgen(module = "/src/koto-highlight-rules.js")]
extern "C" {
    fn register_koto_editor_mode();
}

struct Script {
    name: &'static str,
    script: &'static str,
}

struct ScriptGroup {
    name: &'static str,
    scripts: &'static [Script],
}

const SCRIPTS: &[ScriptGroup] = &[
    ScriptGroup {
        name: "Examples",
        scripts: &[Script {
            name: "Fizz Buzz",
            script: include_str!("scripts/examples/fizz_buzz.koto"),
        }],
    },
    ScriptGroup {
        name: "Canvas",
        scripts: &[Script {
            name: "Random Rects",
            script: include_str!("scripts/canvas/random_rects.koto"),
        }],
    },
];

fn setup_script_menu() {
    let document = gloo_utils::document();
    let menu = get_element_by_id("select-script")
        .dyn_into::<HtmlSelectElement>()
        .unwrap();

    let mut scripts = Vec::new();

    for group in SCRIPTS {
        let group_element = document
            .create_element("optgroup")
            .unwrap()
            .dyn_into::<HtmlOptGroupElement>()
            .unwrap();
        group_element.set_label(group.name);

        for script in group.scripts {
            let option = document
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            option.set_text(script.name);
            option.set_default_selected(false);
            group_element.append_child(&option).unwrap();
            scripts.push(script.script);
        }

        menu.append_child(&group_element)
            .expect("Failed to append script group");
    }

    EventListener::new(&menu.clone(), "change", {
        move |_| {
            console_log!("<<<Script menu changed>>>");
            let script_index = menu.selected_index();
            if script_index > 0 {
                APP.with(|app| app.borrow_mut().reset());

                let ace = get_ace();
                let editor = ace.edit("editor");
                let session = editor.get_session();
                session.set_value(scripts[script_index as usize - 1]);
            }
        }
    })
    .forget();
}

fn setup_editor() {
    let ace = get_ace();
    let editor = ace.edit("editor");
    editor.set_theme("ace/theme/solarized_dark");
    editor.set_show_print_margin(false);

    let session = editor.get_session();
    session.set_mode("ace/mode/koto");
    session.set_value(include_str!("scripts/canvas/random_rects.koto"));

    let on_change =
        Closure::wrap(
            Box::new(|| APP.with(move |app| app.borrow_mut().on_script_edited())) as Box<dyn Fn()>,
        );
    session.on("change", on_change.as_ref().unchecked_ref());
    on_change.forget();
}

fn setup_app() {
    APP.with(|app| app.borrow_mut().on_script_edited());
}

fn get_element_by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Failed to get div with id '{id}'"))
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&JsValue::from(format_args!($($t)*).to_string())))
}
