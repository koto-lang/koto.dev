use {
    super::{script_menu::ScriptMenu, toolbar_button::ToolbarButton},
    yew::prelude::*,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub script_playing: bool,
    pub vim_bindings_enabled: bool,
    pub on_play_clicked: Callback<()>,
    pub on_reload_clicked: Callback<()>,
    pub on_vim_bindings_clicked: Callback<()>,
    pub on_script_selected: Callback<&'static str>,
}

#[function_component(EditorToolbar)]
pub fn editor_toolbar(props: &Props) -> Html {
    let button_size = 0.8;
    html! {
        <div class="editor-toolbar">
            <div class="horizontal-spacer"></div>

            <ToolbarButton
                icon="play"
                size={button_size}
                toggled={props.script_playing}
                on_clicked={props.on_play_clicked.clone()}
            />

            <ToolbarButton
                icon="refresh"
                size={button_size}
                on_clicked={props.on_reload_clicked.clone()}
            />

            <div class="horizontal-spacer"></div>

            <ToolbarButton
                icon="vimeo"
                size={button_size}
                toggled={props.vim_bindings_enabled}
                on_clicked={props.on_vim_bindings_clicked.clone()}
            />

            <div class="horizontal-spacer"></div>

            <ScriptMenu on_script_selected={props.on_script_selected.clone()} />
        </div>
    }
}
