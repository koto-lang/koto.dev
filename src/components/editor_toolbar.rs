use {super::script_menu::ScriptMenu, yew::prelude::*};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub vim_bindings_enabled: bool,
    pub on_vim_bindings_clicked: Callback<()>,
    pub on_script_selected: Callback<&'static str>,
}

#[function_component(EditorToolbar)]
pub fn editor_toolbar(props: &Props) -> Html {
    let vim_bindings_toggle = html! {
        <button
            class={
                if props.vim_bindings_enabled {
                    "uk-button-primary"
                } else {
                    "uk-button-default"
                }
            }
            onclick={
                Callback::from({
                    let on_clicked = props.on_vim_bindings_clicked.clone();
                    move |_| on_clicked.emit(())
                })
            }
        >
            <span uk-icon="icon: vimeo"></span>
        </button>
    };

    html! {
        <div class="editor-toolbar">
            <div class="horizontal-spacer"></div>

            { vim_bindings_toggle }

            <ScriptMenu on_script_selected={props.on_script_selected.clone()} />
        </div>
    }
}
