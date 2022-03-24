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
    pub on_share_clicked: Callback<()>,
    pub on_script_selected: Callback<&'static str>,
}

#[function_component(EditorToolbar)]
pub fn editor_toolbar(props: &Props) -> Html {
    html! {
        <div class="editor-toolbar">
            <ToolbarButton
                icon_left="play"
                caption="Run"
                tooltip="Run the script"
                toggled={props.script_playing}
                on_clicked={props.on_play_clicked.clone()}
            />

            <ToolbarButton
                icon_left="refresh"
                caption="Reload"
                tooltip="Reload the script"
                on_clicked={props.on_reload_clicked.clone()}
            />

            <div class="horizontal-spacer"></div>

            <ToolbarButton
                icon_left="vimeo"
                tooltip="Toggle Vim keybindings"
                toggled={props.vim_bindings_enabled}
                on_clicked={props.on_vim_bindings_clicked.clone()}
            />
            <div class="horizontal-spacer"></div>

            <ToolbarButton
                icon_left="link"
                caption="Share"
                tooltip="Copy a shareable link to the clipboard"
                on_clicked={props.on_share_clicked.clone()}
            />

            <ScriptMenu on_script_selected={props.on_script_selected.clone()} />
        </div>
    }
}
