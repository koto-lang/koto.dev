use {super::toolbar_button::ToolbarButton, cloned::cloned, yew::prelude::*};

struct Script {
    name: &'static str,
    url: &'static str,
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
                    url: "/play/examples/intro/fizz_buzz.koto",
                }],
            },
            ScriptGroup {
                name: "Canvas",
                scripts: &[
                    Script {
                        name: "Alignment",
                        url: "/play/examples/canvas/alignment.koto",
                    },
                    Script {
                        name: "Boids",
                        url: "/play/examples/canvas/boids.koto",
                    },
                    Script {
                        name: "Random Rects",
                        url: "/play/examples/canvas/random_rects.koto",
                    },
                ],
            },
        ]
    });

    let menu_items = use_ref(|| {
        let on_menu_item_clicked = Callback::from({
            cloned!(script_groups, props.on_script_selected);
            move |index| {
                let script_url = script_groups
                    .iter()
                    .flat_map(|script_group| script_group.scripts.iter())
                    .nth(index as usize)
                    .unwrap()
                    .url;
                on_script_selected.emit(script_url);
            }
        });

        let mut script_index = 0;

        html! {
            <ul class="uk-nav uk-nav-default">
            {
                for script_groups.iter().map(|script_group|{
                    html! {
                        <>
                            <li class="uk-nav-header">{script_group.name.to_string()}</li>

                            {
                                for script_group.scripts.iter().map(|script| {
                                    let index = script_index;
                                    script_index += 1;
                                    let onclick = Callback::from({
                                        cloned!(on_menu_item_clicked);
                                        move |_| on_menu_item_clicked.emit(index)
                                    });
                                    html! {
                                        <li>
                                            <a {onclick}>
                                                {script.name.to_string()}
                                            </a>
                                        </li>
                                    }
                                })
                            }
                        </>
                    }
                })
            }
            </ul>
        }
    });

    html! {
        <div class="uk-inline">
            <ToolbarButton
                icon_left="thumbnails"
                icon_right="chevron-down"
                caption="Examples"
                tooltip="Load an example script"
            />

            <div uk-dropdown="mode: click" uk-toggle="true">
                { (*menu_items).clone() }
            </div>
        </div>
    }
}
