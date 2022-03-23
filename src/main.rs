mod ace_bindings;
mod components;
mod koto_wrapper;

use {
    components::playground::Playground, console_error_panic_hook::set_once as set_panic_hook,
    gloo_utils::window, wasm_bindgen::prelude::*, yew::prelude::*,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    set_panic_hook();
    register_koto_editor_mode();
    yew::start_app::<App>();
}

#[wasm_bindgen(module = "/src/koto-highlight-rules.js")]
extern "C" {
    fn register_koto_editor_mode();
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
