use {crate::AppContext, web_sys::HtmlSelectElement, yew::prelude::*};

#[derive(Properties, PartialEq)]
pub struct ScriptMenuProps {
    pub on_script_selected: Callback<&'static str>,
}

#[function_component(ScriptMenu)]
pub fn script_menu(props: &ScriptMenuProps) -> Html {
    let context = use_context::<AppContext>().expect("Missing context");

    let scripts = use_ref(|| {
        let mut scripts = Vec::new();
        let menu_items = html! {
            for context.scripts.iter().map(|script_group|{
                html! {
                    <optgroup label={script_group.name.to_string()}>
                        {
                            for script_group.scripts.iter().map(|script| {
                                scripts.push(script.script);

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
        (menu_items, scripts)
    });

    let onchange = Callback::from({
        let on_script_selected = props.on_script_selected.clone();
        let scripts = scripts.clone();
        move |event: Event| {
            let menu: HtmlSelectElement = event.target_unchecked_into();
            let index = menu.selected_index();
            if index > 0 {
                let script = scripts.1[index as usize - 1];
                on_script_selected.emit(script);
            }
        }
    });

    html! {
        <select {onchange}>
            <option>{"..."}</option>

            { (*scripts).0.clone() }
        </select>
    }
}
