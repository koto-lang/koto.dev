use {
    super::{examples::ExamplesDialog, toolbar_button::ToolbarButton},
    yew::prelude::*,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub auto_run: bool,
    pub on_run_clicked: Callback<()>,
    pub on_auto_run_clicked: Callback<()>,
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
                on_clicked={props.on_run_clicked.clone()}
            />

            <ToolbarButton
                icon_left="refresh"
                caption="Watch"
                tooltip="Automatically run the script when it's changed"
                toggled={props.auto_run}
                on_clicked={props.on_auto_run_clicked.clone()}
            />

            <div class="horizontal-spacer"></div>

            <ToolbarButton
                icon_left="link"
                caption="Share"
                tooltip="Get shareable links for the current script"
                on_clicked={props.on_share_clicked.clone()}
            />

            <ExamplesDialog on_script_selected={props.on_script_selected.clone()} />
        </div>
    }
}
