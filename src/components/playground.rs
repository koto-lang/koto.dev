use {
    super::{editor::Editor, editor_toolbar::EditorToolbar, share::Share},
    crate::{
        ace_bindings::AceEditor, koto_wrapper::KotoWrapper, show_notification,
        stored_value::StoredValue,
    },
    gloo_events::EventListener,
    gloo_net::http::Request,
    gloo_timers::callback::Interval,
    gloo_utils::window,
    serde::Deserialize,
    std::collections::HashMap,
    web_sys::{Element, HtmlCanvasElement, UrlSearchParams},
    yew::prelude::*,
};

pub enum Msg {
    EditorInitialized { editor: AceEditor },
    EditorChanged,
    ScriptLoaded { contents: String },
    ScriptMenuChanged { url: &'static str },
    PlayButtonClicked,
    ReloadButtonClicked,
    ShareButtonClicked,
    ShareModalClosed,
    ToggleVimBindings,
    ToggleEditorTheme,
    OnUpdate,
    WindowResized,
    BeforeUnload,
    SetFps(f64),
    ShowError { error: String },
}

pub struct Playground {
    canvas_ref: NodeRef,
    compiler_output_ref: NodeRef,
    script_output_ref: NodeRef,

    editor: Option<AceEditor>,
    koto: Option<KotoWrapper>,

    run_script_enabled: bool,

    script: StoredValue<String>,
    vim_bindings_enabled: StoredValue<bool>,
    light_theme_enabled: StoredValue<bool>,

    update_interval: Option<Interval>,
    last_time: Option<f64>,
    current_time: f64,
    update_fps: f64,

    show_share_dialog: bool,

    _event_listeners: Vec<EventListener>,
}

impl Playground {
    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("Missing canvas element")
    }

    fn setup_update_interval(&mut self, ctx: &Context<Self>) {
        let interval_ms = (1.0 / self.update_fps * 1000.0).floor() as u32;
        self.update_interval = Some(Interval::new(interval_ms, {
            let link = ctx.link().clone();
            move || link.send_message(Msg::OnUpdate)
        }));
    }

    fn get_koto(&mut self) -> &mut KotoWrapper {
        self.koto.as_mut().expect("Missing koto wrapper")
    }

    fn get_editor(&self) -> &AceEditor {
        self.editor.as_ref().expect("Missing editor")
    }

    fn get_editor_contents(&self) -> String {
        self.get_editor().get_session().get_value()
    }

    fn set_editor_contents(&mut self, contents: &str) {
        self.get_editor().get_session().set_value(contents);
    }

    fn reset(&mut self) {
        self.get_koto().reset();
        self.update_interval = None;
        self.current_time = 0.0;
        self.last_time = None;
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
                self.script.clone()
            }
        };

        self.set_editor_contents(&script);
        self.set_vim_bindings_enabled(*self.vim_bindings_enabled);
        self.set_light_theme_enabled(*self.light_theme_enabled);
    }

    fn set_light_theme_enabled(&mut self, enabled: bool) {
        self.light_theme_enabled.set(enabled);
        self.get_editor().set_theme(if enabled {
            "ace/theme/solarized_light"
        } else {
            "ace/theme/solarized_dark"
        });
    }

    fn set_vim_bindings_enabled(&mut self, enabled: bool) {
        self.vim_bindings_enabled.set(enabled);
        self.get_editor()
            .set_keyboard_handler(if enabled { "ace/keyboard/vim" } else { "" });
    }

    fn run_script(&mut self, ctx: &Context<Self>) {
        debug_assert!(self.run_script_enabled);

        let update_interval_is_none = self.update_interval.is_none();
        let koto = self.get_koto();

        if koto.is_ready() && !koto.is_initialized() {
            koto.run();
        }

        // koto.is_ready() is re-checked here in case an error occurred during koto.run()
        if koto.is_ready() && update_interval_is_none && koto.update_should_be_called() {
            self.setup_update_interval(ctx)
        }
    }
}

impl Component for Playground {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            compiler_output_ref: NodeRef::default(),
            script_output_ref: NodeRef::default(),
            editor: None,
            koto: None,
            script: StoredValue::new_with_default("script", || {
                include_str!("../../examples/canvas/random_rects.koto").into()
            }),
            light_theme_enabled: StoredValue::new("light_theme_enabled"),
            vim_bindings_enabled: StoredValue::new("vim-bindings-enabled"),
            update_interval: None,
            last_time: None,
            current_time: 0.0,
            update_fps: 60.0,
            run_script_enabled: true,
            show_share_dialog: false,
            _event_listeners: vec![
                EventListener::new(&window(), "resize", {
                    let link = ctx.link().clone();
                    move |_| link.send_message(Msg::WindowResized)
                }),
                EventListener::new(&window(), "beforeunload", {
                    let link = ctx.link().clone();
                    move |_| link.send_message(Msg::BeforeUnload)
                }),
            ],
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
                let script = self.get_editor_contents();
                if !script.is_empty() {
                    let koto = self.get_koto();
                    koto.compile_script(&script);
                    if self.run_script_enabled {
                        self.run_script(ctx);
                    }
                }
                self.script.set(script.into());
                true
            }
            Msg::ScriptLoaded { contents } => {
                self.reset();
                self.set_editor_contents(&contents);
                false
            }
            Msg::ScriptMenuChanged { url } => {
                ctx.link().send_future({
                    async {
                        match Request::get(url).send().await {
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
            Msg::PlayButtonClicked => {
                self.run_script_enabled = !self.run_script_enabled;
                if self.run_script_enabled {
                    self.run_script(ctx);
                } else {
                    self.update_interval = None;
                }
                true
            }
            Msg::ReloadButtonClicked => {
                self.reset();
                ctx.link().send_message(Msg::EditorChanged);
                false
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
            Msg::ToggleEditorTheme => {
                self.set_light_theme_enabled(!*self.light_theme_enabled);
                true
            }
            Msg::ToggleVimBindings => {
                self.set_vim_bindings_enabled(!*self.vim_bindings_enabled);
                true
            }
            Msg::OnUpdate => {
                let time = get_current_time();
                let time_delta = time - self.last_time.unwrap_or(time);
                self.current_time += time_delta;
                let current_time = self.current_time;
                self.last_time = Some(time);

                let koto = self.get_koto();
                if koto.is_ready() {
                    koto.run_update(current_time);
                }

                // is_ready gets checked again here in case of an error when running update()
                if koto.is_ready() {
                    false
                } else {
                    self.update_interval = None;
                    true
                }
            }
            Msg::SetFps(fps) => {
                // If the update interval is currently active, then restart it with the new fps.
                // If it's not currelntly active then it'll be set up later in self.run_script().
                let restart_interval = self.update_fps != fps && self.update_interval.is_some();

                self.update_fps = fps;
                if restart_interval {
                    self.setup_update_interval(ctx);
                }

                false
            }
            Msg::WindowResized => {
                let canvas = self.get_canvas();
                canvas.set_width(canvas.client_width() as u32);
                canvas.set_height(canvas.client_height() as u32);
                self.get_koto().on_resize();
                false
            }
            Msg::BeforeUnload => {
                self.script.save();
                self.light_theme_enabled.save();
                self.vim_bindings_enabled.save();
                false
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
                    script_playing={self.run_script_enabled}
                    light_theme_enabled={*self.light_theme_enabled}
                    vim_bindings_enabled={*self.vim_bindings_enabled}
                    on_play_clicked={ctx.link().callback(|_| Msg::PlayButtonClicked)}
                    on_reload_clicked={ctx.link().callback(|_| Msg::ReloadButtonClicked)}
                    on_theme_clicked={ctx.link().callback(|_| Msg::ToggleEditorTheme)}
                    on_vim_bindings_clicked={ctx.link().callback(|_| Msg::ToggleVimBindings)}
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
            <>
                <div class="playground">
                    { editor_area }

                    <canvas
                      ref={self.canvas_ref.clone()}
                      class="playground-canvas fullsize"
                      width="400"
                      height="400"
                    ></canvas>

                    <textarea
                      ref={self.compiler_output_ref.clone()}
                      class="fixed-mono"
                      readonly=true
                    ></textarea>

                    <textarea
                      ref={self.script_output_ref.clone()}
                      class="fixed-mono"
                      readonly=true
                    ></textarea>
                </div>

                {
                    if self.show_share_dialog {
                        html! {
                            <Share
                                script={self.script.clone()}
                                on_hidden={ctx.link().callback(|_| Msg::ShareModalClosed)}
                            />
                        }
                    } else {
                        html! {}
                    }
                }
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.get_canvas();

            canvas.set_width(canvas.client_width() as u32);
            canvas.set_height(canvas.client_height() as u32);

            let compiler_output = self.compiler_output_ref.cast::<Element>().unwrap();
            let script_output = self.script_output_ref.cast::<Element>().unwrap();

            self.koto = Some(KotoWrapper::new(canvas, compiler_output, script_output, {
                ctx.link().callback(move |fps| Msg::SetFps(fps))
            }));
        }

        self.show_share_dialog = false;
    }
}

// Returns the current time in seconds
fn get_current_time() -> f64 {
    window().performance().unwrap().now() / 1000.0
}

#[derive(Deserialize)]
struct Gist {
    files: HashMap<String, GistFile>,
}

#[derive(Deserialize)]
struct GistFile {
    content: String,
}
