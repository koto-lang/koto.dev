mod ace_bindings;
mod components;
mod koto_wrapper;
mod stored_value;

use {
    components::playground::Playground,
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_utils::{document, window},
    wasm_bindgen::prelude::*,
    yew::prelude::*,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    set_panic_hook();

    register_koto_editor_mode();

    let playground_wrapper = document()
        .get_element_by_id("playground-wrapper")
        .expect("Missing playground wrapper");

    yew::start_app_in_element::<App>(playground_wrapper);
}

#[wasm_bindgen(module = "/src/koto-highlight-rules.js")]
extern "C" {
    fn register_koto_editor_mode();
}

#[wasm_bindgen(module = "/src/show_notification.js")]
extern "C" {
    fn show_notification(message: &str, icon: &str);
}

#[wasm_bindgen(module = "/src/copy_text_to_clipboard.js")]
extern "C" {
    fn copy_text_to_clipboard(text: &str);
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <Playground />
        </div>
    }
}

fn get_local_storage_value(id: &str) -> Option<String> {
    window()
        .local_storage()
        .expect("Couldn't access local storage")
        .map_or(None, |storage| {
            storage
                .get(id)
                .expect("Couldn't get item from local storage")
        })
}

fn set_local_storage_value(id: &str, value: &str) {
    window()
        .local_storage()
        .expect("Couldn't access local storage")
        .expect("Missing local storage")
        .set(id, &value)
        .ok();
}
