mod ace_bindings;
mod components;
mod koto_wrapper;

use {
    components::playground::Playground, console_error_panic_hook::set_once as set_panic_hook,
    gloo_utils::window, std::rc::Rc, wasm_bindgen::prelude::*, yew::prelude::*,
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
    let context = use_ref(|| AppContext {
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
                        name: "Alignment",
                        script: include_str!("scripts/canvas/alignment.koto"),
                    },
                    Script {
                        name: "Boids",
                        script: include_str!("scripts/canvas/boids.koto"),
                    },
                    Script {
                        name: "Random Rects",
                        script: include_str!("scripts/canvas/random_rects.koto"),
                    },
                ],
            },
        ]
        .into(),
    });

    html! {
        <ContextProvider<AppContext> context={(*context).clone()}>
            <div class="container">
                <Playground />
            </div>
        </ContextProvider<AppContext>>
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

struct Script {
    name: &'static str,
    script: &'static str,
}

struct ScriptGroup {
    name: &'static str,
    scripts: &'static [Script],
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
