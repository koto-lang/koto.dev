mod ace_bindings;
mod components;
mod koto_wrapper;
mod stored_value;

use {
    components::playground::Playground,
    gloo_utils::{body, document, window},
    js_sys::encode_uri_component,
    wasm_bindgen::prelude::*,
    web_sys::console,
    yew::prelude::*,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::set_custom_panic_hook(Box::new(custom_panic_hook));

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
        .and_then(|storage| {
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
        .set(id, value)
        .ok();
}

fn custom_panic_hook(info: &std::panic::PanicHookInfo) {
    let message = info.to_string(); // Get the panic message
    let backtrace = info
        .location()
        .unwrap_or_else(|| panic!("Failed to get location")); // Get the backtrace location

    let error_message = format!("{message}\n{backtrace:?}");

    // Create a GitHub issue link
    let issue_body = format!(
        ">> Please describe what you were doing when this error occurred <<

**Error:**
```
{error_message}
```"
    );
    let issue_url = format!(
        "https://github.com/koto-lang/koto.dev/issues/new?title=Playground+Crash&body={}",
        encode_uri_component(&issue_body)
    );

    // Log the error in the console
    console::log_1(&error_message.into());

    // Update the HTML body
    let html = format!(
        "
<div class='uk-container uk-padding'>
    <div class='uk-card uk-card-default uk-margin'>
        <div class='uk-card-body uk-alert-primary uk-text-lead uk-text-danger uk-text-center'>
            An unexpected error occurred. 
            <br/>
            Please <a class='uk-link-text' href='{issue_url}' target='_blank'>click here</a> to report this issue.
        </div>
    </div>
</div>
",
    );
    if let Some(wrapper) = document().get_element_by_id("playground-wrapper") {
        wrapper.set_inner_html(&html);
    } else {
        body().set_inner_html(&html);
    }
}
