use {crate::AppContext, web_sys::HtmlSelectElement, yew::prelude::*};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_script_selected: Callback<&'static str>,
}

#[function_component(ScriptMenu)]
pub fn script_menu(props: &Props) -> Html {
    let context = use_context::<AppContext>().expect("Missing context");

    let menu_items = use_ref(|| {
        let menu_items = html! {
            for context.scripts.iter().map(|script_group|{
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
        };
        menu_items
    });

    let onchange = Callback::from({
        let scripts = context.scripts.clone();
        let on_script_selected = props.on_script_selected.clone();
        move |event: Event| {
            let menu: HtmlSelectElement = event.target_unchecked_into();
            let index = menu.selected_index();
            if index > 0 {
                let script = scripts
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
        <select {onchange}>
            <option>{"Example Scripts"}</option>

            { (*menu_items).clone() }
        </select>
    }
}
