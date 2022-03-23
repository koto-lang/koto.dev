use {cloned::cloned, web_sys::HtmlSelectElement, yew::prelude::*};

struct Script {
    name: &'static str,
    script: &'static str,
}

struct ScriptGroup {
    name: &'static str,
    scripts: &'static [Script],
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_script_selected: Callback<&'static str>,
}

#[function_component(ScriptMenu)]
pub fn script_menu(props: &Props) -> Html {
    let script_groups = use_ref(|| {
        vec![
            ScriptGroup {
                name: "Examples",
                scripts: &[Script {
                    name: "Fizz Buzz",
                    script: include_str!("../scripts/examples/fizz_buzz.koto"),
                }],
            },
            ScriptGroup {
                name: "Canvas",
                scripts: &[
                    Script {
                        name: "Alignment",
                        script: include_str!("../scripts/canvas/alignment.koto"),
                    },
                    Script {
                        name: "Boids",
                        script: include_str!("../scripts/canvas/boids.koto"),
                    },
                    Script {
                        name: "Random Rects",
                        script: include_str!("../scripts/canvas/random_rects.koto"),
                    },
                ],
            },
        ]
    });

    let menu_items = use_ref(|| {
        html! {
            for script_groups.iter().map(|script_group|{
                html! {
                    <optgroup label={script_group.name.to_string()}>
                        {
                            for script_group.scripts.iter().map(|script| {
                                html! {
                                    <option>
                                        {script.name.to_string()}
                                    </option>
                                }
                            })
                        }
                    </optgroup>
                }
            })
        }
    });

    let onchange = Callback::from({
        cloned!(script_groups, props.on_script_selected);
        move |event: Event| {
            let menu: HtmlSelectElement = event.target_unchecked_into();
            let index = menu.selected_index();
            if index > 0 {
                let script = script_groups
                    .iter()
                    .flat_map(|script_group| script_group.scripts.iter())
                    .nth(index as usize - 1)
                    .unwrap()
                    .script;
                on_script_selected.emit(script);
            }
        }
    });

    html! {
        <select
            uk-tooltip="title: Example Koto Scripts; delay: 500"
            {onchange}
        >
            <option>{"Examples"}</option>

            { (*menu_items).clone() }
        </select>
    }
}
