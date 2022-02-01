#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use {console_error_panic_hook::set_once as set_panic_hook, web_sys::window};

fn main() {
    set_panic_hook();
    run_app();
}

fn run_app() {
    let document = window()
        .and_then(|w| w.document())
        .expect("Failed to access document");
    let body = document.body().expect("Failed to access document.body");
    let text_node = document.create_text_node("Hello, world!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}
