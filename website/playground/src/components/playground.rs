use {
    super::{editor::Editor, editor_toolbar::EditorToolbar, share::Share},
    crate::{
        ace_bindings::AceEditor, koto_wrapper::KotoWrapper, show_notification,
        stored_value::StoredValue,
    },
    // gloo_console::log,
    gloo_events::EventListener,
    gloo_net::http::Request,
    gloo_utils::{document, window},
    serde::Deserialize,
    std::collections::HashMap,
    wasm_bindgen::{closure::Closure, JsCast},
    web_sys::{Element, MutationObserver, MutationObserverInit, RequestCache, UrlSearchParams},
    yew::prelude::*,
};

#[derive(PartialEq, Clone, Copy)]
pub struct PlaygroundContext {
    pub dark_mode: bool,
}

pub enum Msg {
    EditorInitialized { editor: AceEditor },
    EditorChanged,
    ScriptLoaded { contents: String },
    PostScriptLoaded,
    ScriptMenuChanged { url: &'static str },
    RunButtonClicked,
    AutoRunButtonClicked,
    ShareButtonClicked,
    ShareModalClosed,
    BeforeUnload,
    DocumentAttributesChanged,
    ShowError { error: String },
}

pub struct Playground {
    playground_context: PlaygroundContext,

    script_output_ref: NodeRef,

    editor: Option<AceEditor>,
    koto: Option<KotoWrapper>,

    auto_run_enabled: bool,

    script: StoredValue<String>,
    vim_bindings_enabled: StoredValue<bool>,

    show_share_dialog: bool,

    ignore_editor_changed: bool,

    _event_listeners: Vec<EventListener>,
    _document_attributes_listener: MutationObserver,
    _document_attributes_callback: Closure<dyn FnMut()>,
}

impl Playground {
    fn get_editor(&self) -> &AceEditor {
        self.editor.as_ref().expect("Missing editor")
    }

    fn get_editor_contents(&self) -> String {
        self.get_editor().get_session().get_value()
    }

    fn set_editor_contents(&mut self, contents: &str) {
        self.get_editor().get_session().set_value(contents);
    }

    fn setup_editor(&mut self, ctx: &Context<Self>) {
        let script = {
            let url_params = UrlSearchParams::new_with_str(
                &window()
                    .location()
                    .search()
                    .expect("Missing location search string"),
            )
            .expect("Failed to create UrlSearchParams");

            if let Some(gist) = url_params.get("gist") {
                ctx.link().send_future(async move {
                    match Request::get(&format!("https://api.github.com/gists/{gist}"))
                        .send()
                        .await
                    {
                        Ok(response) => match response.json::<Gist>().await {
                            Ok(gist) => match gist.files.values().next() {
                                Some(file) => Msg::ScriptLoaded {
                                    contents: file.content.clone(),
                                },
                                None => Msg::ShowError {
                                    error: "The gist doesn't contain any files".into(),
                                },
                            },
                            Err(_) => Msg::ShowError {
                                error: "Failed to load gist".into(),
                            },
                        },
                        Err(error) => Msg::ShowError {
                            error: format!("Failed to access gist (error: '{error}')"),
                        },
                    }
                });
                "".into()
            } else if let Some(script) = url_params.get("script") {
                script.into()
            } else {
                self.script.as_ref().clone()
            }
        };

        self.set_editor_contents(&script);
        self.set_vim_bindings_enabled(self.vim_bindings_enabled.get());
        self.update_editor_theme();
    }

    fn update_editor_theme(&self) {
        self.get_editor()
            .set_theme(if self.playground_context.dark_mode {
                "ace/theme/solarized_dark"
            } else {
                "ace/theme/solarized_light"
            });
    }

    // This was available via a UI toggle, but can now be enabled via:
    // document.documentElement.setAttribute("editor-bindings", "vim");
    //
    // At some point a settings dialog will be added and vim bindings can be re-enabled,
    // alongside other bindings, e.g. emacs.
    fn set_vim_bindings_enabled(&mut self, enabled: bool) {
        self.vim_bindings_enabled.set(enabled);
        self.get_editor()
            .set_keyboard_handler(if enabled { "ace/keyboard/vim" } else { "" });
    }

    fn compile_and_run_script(&mut self) {
        let koto = self.koto.as_mut().expect("Missing koto wrapper");
        koto.compile_and_run_script(&self.script.as_ref());
    }
}

impl Component for Playground {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dark_mode = match document()
            .document_element()
            .expect("Missing document element")
            .get_attribute("color-scheme")
        {
            Some(scheme) if scheme == "dark" => true,
            _ => false,
        };

        let document_attributes_callback = Closure::wrap({
            let link = ctx.link().clone();
            Box::new(move || link.send_message(Msg::DocumentAttributesChanged)) as Box<dyn FnMut()>
        });
        let document_attributes_listener =
            MutationObserver::new(document_attributes_callback.as_ref().unchecked_ref()).unwrap();
        let mut observation_options = MutationObserverInit::new();
        observation_options.attributes(true);
        document_attributes_listener
            .observe_with_options(
                &document()
                    .document_element()
                    .expect("Missing document element"),
                &observation_options,
            )
            .expect("Failed to add document attributes observer");

        let playground_context = PlaygroundContext { dark_mode };

        Self {
            playground_context,
            script_output_ref: NodeRef::default(),
            editor: None,
            koto: None,
            script: StoredValue::new_with_default("script", || {
                include_str!("../../examples/intro/fizz_buzz.koto").into()
            }),
            vim_bindings_enabled: StoredValue::new("vim-bindings-enabled"),
            auto_run_enabled: true,
            show_share_dialog: false,
            ignore_editor_changed: false,
            _event_listeners: vec![EventListener::new(&window(), "beforeunload", {
                let link = ctx.link().clone();
                move |_| link.send_message(Msg::BeforeUnload)
            })],
            _document_attributes_listener: document_attributes_listener,
            _document_attributes_callback: document_attributes_callback,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EditorInitialized { editor } => {
                self.editor = Some(editor);
                self.setup_editor(ctx);
                false
            }
            Msg::EditorChanged => {
                if self.ignore_editor_changed {
                    false
                } else {
                    let script = self.get_editor_contents();
                    self.script.set(script.into());
                    if self.auto_run_enabled {
                        self.compile_and_run_script();
                    }
                    true
                }
            }
            Msg::ScriptLoaded { contents } => {
                // We only want to compile the script once, and the Ace editor can send multiple
                // on_changed events when setting its contents.
                self.ignore_editor_changed = true;
                self.set_editor_contents(&contents);
                ctx.link().send_message(Msg::PostScriptLoaded);
                true
            }
            Msg::PostScriptLoaded => {
                self.ignore_editor_changed = false;
                ctx.link().send_message(Msg::EditorChanged);
                false
            }
            Msg::ScriptMenuChanged { url } => {
                ctx.link().send_future({
                    async {
                        match Request::get(url).cache(RequestCache::NoCache).send().await {
                            Ok(response) => match response.text().await {
                                Ok(contents) => Msg::ScriptLoaded { contents },
                                Err(_) => Msg::ShowError {
                                    error: "Failed to load example script".to_string(),
                                },
                            },
                            Err(_) => Msg::ShowError {
                                error: "Failed to load example".to_string(),
                            },
                        }
                    }
                });
                false
            }
            Msg::RunButtonClicked => {
                self.compile_and_run_script();
                true
            }
            Msg::AutoRunButtonClicked => {
                self.auto_run_enabled = !self.auto_run_enabled;
                if self.auto_run_enabled {
                    self.compile_and_run_script();
                }
                true
            }
            Msg::ShareButtonClicked => {
                // self.copy_link_to_clipboard();
                self.show_share_dialog = true;
                true
            }
            Msg::ShareModalClosed => {
                self.show_share_dialog = false;
                true
            }
            Msg::BeforeUnload => {
                self.script.save();
                self.vim_bindings_enabled.save();
                false
            }
            Msg::DocumentAttributesChanged => {
                let document_element = document()
                    .document_element()
                    .expect("Missing document element");

                let dark_mode = match document_element.get_attribute("color-scheme") {
                    Some(scheme) if scheme == "dark" => true,
                    _ => false,
                };

                let dark_mode_changed = self.playground_context.dark_mode != dark_mode;
                if dark_mode_changed {
                    self.playground_context.dark_mode = dark_mode;
                    self.update_editor_theme();
                }

                let vim_bindings_enabled = match document_element.get_attribute("editor-bindings") {
                    Some(scheme) if scheme == "vim" => true,
                    _ => false,
                };
                let vim_bindings_enabled_changed =
                    self.vim_bindings_enabled.get() != vim_bindings_enabled;
                if vim_bindings_enabled_changed {
                    self.set_vim_bindings_enabled(vim_bindings_enabled);
                }

                dark_mode_changed
            }
            Msg::ShowError { error } => {
                show_notification(&error, "error");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let editor_area = html! {
            <div class="editor-area">
                <EditorToolbar
                    auto_run={self.auto_run_enabled}
                    on_run_clicked={ctx.link().callback(|_| Msg::RunButtonClicked)}
                    on_auto_run_clicked={ctx.link().callback(|_| Msg::AutoRunButtonClicked)}
                    on_share_clicked={ctx.link().callback(|_| Msg::ShareButtonClicked)}
                    on_script_selected={
                        ctx.link().callback(|url| Msg::ScriptMenuChanged {url})
                    }
                />

                <Editor
                    on_initialized={ctx.link().callback(|editor| Msg::EditorInitialized {editor})}
                    on_changed={ctx.link().callback(|_| Msg::EditorChanged)}
                />
            </div>
        };

        html! {
            <ContextProvider<PlaygroundContext> context={self.playground_context}>
                <div class="playground">
                    { editor_area }

                    <textarea
                      ref={self.script_output_ref.clone()}
                      class="playground-output fixed-mono uk-textarea uk-form-small"
                      readonly=true
                    ></textarea>
                </div>

                {
                    if self.show_share_dialog {
                        html! {
                            <Share
                                script={self.script.as_ref().clone()}
                                on_hidden={ctx.link().callback(|_| Msg::ShareModalClosed)}
                            />
                        }
                    } else {
                        html! {}
                    }
                }
            </ContextProvider<PlaygroundContext>>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let script_output = self.script_output_ref.cast::<Element>().unwrap();

            self.koto = Some(KotoWrapper::new(script_output));
        }

        self.show_share_dialog = false;
    }
}

#[derive(Deserialize)]
struct Gist {
    files: HashMap<String, GistFile>,
}

#[derive(Deserialize)]
struct GistFile {
    content: String,
}
