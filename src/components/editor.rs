use {
    crate::ace_bindings::{get_ace, AceEditor},
    wasm_bindgen::{prelude::*, JsCast},
    yew::prelude::*,
};

pub enum Msg {
    EditorChanged,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_changed: Callback<Box<str>>,
    pub on_initialized: Callback<AceEditor>,
}

pub struct Editor {
    editor_id: String,
    editor: Option<AceEditor>,
    #[allow(dyn_drop)]
    editor_changed_callback: Option<Box<dyn Drop>>,
}

impl Editor {
    fn get_editor_value(&self) -> String {
        self.editor
            .as_ref()
            .expect("Missing editor")
            .get_session()
            .get_value()
    }
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            editor_id: "editor".into(),
            editor: None,
            editor_changed_callback: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EditorChanged => {
                ctx.props()
                    .on_changed
                    .emit(self.get_editor_value().into_boxed_str());
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div
              id={self.editor_id.clone()}
              class="flex-grow"
            >
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let ace = get_ace();
            let editor = ace.edit(&self.editor_id);
            editor.set_theme("ace/theme/solarized_dark");
            editor.set_show_print_margin(false);

            let session = editor.get_session();
            session.set_mode("ace/mode/koto");
            session.set_use_soft_tabs(true);
            session.set_tab_size(2);

            let editor_changed_callback = Closure::wrap({
                let link = ctx.link().clone();
                Box::new(move || link.send_message(Msg::EditorChanged))
            } as Box<dyn FnMut()>);
            session.on("change", editor_changed_callback.as_ref().unchecked_ref());

            self.editor_changed_callback = Some(Box::new(editor_changed_callback));
            self.editor = Some(editor);

            ctx.props().on_initialized.emit(ace.edit(&self.editor_id));
        }
    }
}
