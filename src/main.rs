mod ace_bindings;
mod components;
mod koto_wrapper;

use {
    crate::koto_wrapper::KotoMessageQueue,
    components::playground::Playground,
    console_error_panic_hook::set_once as set_panic_hook,
    gloo_console::log,
    gloo_events::EventListener,
    gloo_utils::{document, window},
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
    wasm_bindgen::prelude::*,
    web_sys::Element,
    yew::prelude::*,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    set_panic_hook();
    register_koto_editor_mode();
    yew::start_app::<App>();
}

struct App {
    context: AppContext,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            context: AppContext {
                scripts: vec![
                    ScriptGroup {
                        name: "Examples",
                        scripts: &[Script {
                            name: "Fizz Buzz",
                            script: include_str!("scripts/examples/fizz_buzz.koto"),
                        }],
                    },
                    ScriptGroup {
                        name: "Canvas",
                        scripts: &[
                            Script {
                                name: "Boids",
                                script: include_str!("scripts/canvas/boids.koto"),
                            },
                            Script {
                                name: "Random Rects",
                                script: include_str!("scripts/canvas/random_rects.koto"),
                            },
                            Script {
                                name: "Alignment",
                                script: include_str!("scripts/canvas/alignment.koto"),
                            },
                        ],
                    },
                ]
                .into(),
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<AppContext> context={self.context.clone()}>
                <div class="container">
                    <h1>{"Playground"}</h1>

                    <Playground />
                </div>
            </ContextProvider<AppContext>>
        }
    }
}

#[derive(Clone)]
struct AppContext {
    scripts: Rc<[ScriptGroup]>,
}

impl PartialEq for AppContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.scripts, &other.scripts)
    }
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

fn get_element_by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Failed to get div with id '{id}'"))
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

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&JsValue::from(format_args!($($t)*).to_string())))
}
