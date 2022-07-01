use {
    crate::ace_bindings::{get_ace, AceEditor},
    cloned::cloned,
    wasm_bindgen::{prelude::*, JsCast},
    yew::prelude::*,
};

// This should be made unique if multiple editors per page are needed
static EDITOR_ID: &'static str = "koto-playground-editor";

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_initialized: Callback<AceEditor>,
    pub on_changed: Callback<()>,
}

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    use_effect_with_deps(
        {
            cloned!(props.on_initialized, props.on_changed);
            move |_| {
                let ace = get_ace();
                let editor = ace.edit(EDITOR_ID);
                editor.set_show_print_margin(false);

                let session = editor.get_session();
                session.set_mode("ace/mode/koto");
                session.set_use_soft_tabs(true);
                session.set_tab_size(2);

                let editor_changed_callback =
                    Closure::wrap({ Box::new(move || on_changed.emit(())) } as Box<dyn FnMut()>);
                session.on("change", editor_changed_callback.as_ref().unchecked_ref());

                on_initialized.emit(ace.edit(EDITOR_ID));

                || drop(editor_changed_callback)
            }
        },
        (),
    );

    html! {
        <div id={EDITOR_ID} class="playground-editor flex-grow"></div>
    }
}

#[wasm_bindgen(module = "/src/copy_text_to_clipboard.js")]
extern "C" {
    fn setup_ace_base_path(text: &str);
}
