mod ace_bindings;
mod app;
mod koto_wrapper;

use {
    crate::{app::App, koto_wrapper::KotoMessageQueue},
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_utils::document,
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
    wasm_bindgen::prelude::*,
    web_sys::Element,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    static KOTO_MESSAGE_QUEUE: KotoMessageQueue = Rc::new(RefCell::new(VecDeque::new()));
}

fn main() {
    set_panic_hook();
    register_koto_editor_mode();
    App::setup();
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

fn get_element_by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Failed to get div with id '{id}'"))
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&JsValue::from(format_args!($($t)*).to_string())))
}
