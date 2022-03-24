use {
    super::{editor::Editor, editor_toolbar::EditorToolbar},
    crate::{
        ace_bindings::AceEditor, copy_text_to_clipboard, get_local_storage_value,
        koto_wrapper::KotoWrapper, set_local_storage_value, show_notification,
    },
    gloo_events::EventListener,
    gloo_render::AnimationFrame,
    gloo_utils::window,
    js_sys::{decode_uri_component, encode_uri_component},
    web_sys::{Element, HtmlCanvasElement, UrlSearchParams},
    yew::prelude::*,
};

pub enum Msg {
    EditorInitialized { editor: AceEditor },
    EditorChanged,
    ScriptMenuChanged { script: &'static str },
    PlayButtonClicked,
    ReloadButtonClicked,
    ShareButtonClicked,
    ToggleVimBindings,
    AnimationFrame { time: f64 },
    WindowResized,
    BeforeUnload,
}

pub struct Playground {
    canvas_ref: NodeRef,
    compiler_output_ref: NodeRef,
    script_output_ref: NodeRef,

    editor: Option<AceEditor>,
    koto: Option<KotoWrapper>,

    script: Box<str>,
    vim_bindings_enabled: bool,

    run_script_enabled: bool,
    animation_frame: Option<AnimationFrame>,
    last_time: Option<f64>,
    current_time: f64,

    _event_listeners: Vec<EventListener>,
}

impl Playground {
    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("Missing canvas element")
    }

    fn request_animation_frame(&mut self, ctx: &Context<Self>) {
        self.animation_frame = Some(gloo_render::request_animation_frame({
            let link = ctx.link().clone();
            move |time| link.send_message(Msg::AnimationFrame { time })
        }));
    }

    fn get_koto(&mut self) -> &mut KotoWrapper {
        self.koto.as_mut().expect("Missing koto wrapper")
    }

    fn get_editor(&mut self) -> &AceEditor {
        self.editor.as_ref().expect("Missing editor")
    }

    fn get_editor_contents(&mut self) -> String {
        self.get_editor().get_session().get_value()
    }

    fn set_editor_contents(&mut self, contents: &str) {
        self.get_editor().get_session().set_value(contents);
    }

    fn reset(&mut self) {
        self.get_koto().reset();
        self.animation_frame = None;
        self.current_time = 0.0;
        self.last_time = None;
    }

    fn set_vim_bindings_enabled(&mut self, enabled: bool) {
        self.vim_bindings_enabled = enabled;
        self.get_editor()
            .set_keyboard_handler(if enabled { "ace/keyboard/vim" } else { "" });
    }

    fn run_script(&mut self, ctx: &Context<Self>) {
        debug_assert!(self.run_script_enabled);

        let koto = self.get_koto();

        if koto.is_ready() && !koto.is_initialized() {
            koto.run();
        }

        if koto.is_ready() && koto.update_should_be_called() {
            self.request_animation_frame(ctx)
        }
    }

    fn copy_link_to_clipboard(&self) {
        let location = window().location();
        let origin = location.origin().expect("Missing location origin");
        let path = location.pathname().expect("Missing location pathname");
        let script = encode_uri_component(&self.script);
        let link = format!("{origin}{path}?script={script}");
        copy_text_to_clipboard(&link);
        show_notification("Link copied to clipboard", "link");
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
            script: "".into(),
            vim_bindings_enabled: get_local_storage_value("vim-bindings-enabled")
                .map_or(false, |enabled| enabled == "true"),
            animation_frame: None,
            last_time: None,
            current_time: 0.0,
            run_script_enabled: true,
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
                let script = {
                    let url_params = UrlSearchParams::new_with_str(
                        &window()
                            .location()
                            .search()
                            .expect("Missing location search string"),
                    )
                    .expect("Failed to create UrlSearchParams");

                    if let Some(script) = url_params.get("script") {
                        match decode_uri_component(&script) {
                            Ok(script) => script.into(),
                            Err(_) => {
                                show_notification("Failed to read script from url", "error");
                                "".to_string()
                            }
                        }
                    } else {
                        get_local_storage_value("script").unwrap_or(
                            include_str!("../scripts/canvas/random_rects.koto").to_string(),
                        )
                    }
                };
                self.set_editor_contents(&script);
                self.set_vim_bindings_enabled(self.vim_bindings_enabled);
                false
            }
            Msg::EditorChanged => {
                self.animation_frame = None;
                let script = self.get_editor_contents();
                if !script.is_empty() {
                    let koto = self.get_koto();
                    koto.compile_script(&script);
                    if self.run_script_enabled {
                        self.run_script(ctx);
                    }
                }
                self.script = script.into();
                true
            }
            Msg::ScriptMenuChanged { script } => {
                self.reset();
                self.set_editor_contents(script);
                false
            }
            Msg::PlayButtonClicked => {
                self.run_script_enabled = !self.run_script_enabled;
                if self.run_script_enabled {
                    self.run_script(ctx);
                } else {
                    self.animation_frame = None;
                }
                true
            }
            Msg::ReloadButtonClicked => {
                self.reset();
                ctx.link().send_message(Msg::EditorChanged);
                false
            }
            Msg::ShareButtonClicked => {
                self.copy_link_to_clipboard();
                false
            }
            Msg::ToggleVimBindings => {
                self.set_vim_bindings_enabled(!self.vim_bindings_enabled);
                true
            }
            Msg::AnimationFrame { time } => {
                self.animation_frame = None;

                let time_delta = (time - self.last_time.unwrap_or(time)) / 1000.0;
                self.current_time += time_delta;
                let current_time = self.current_time;
                self.last_time = Some(time);

                let koto = self.get_koto();
                if koto.is_ready() {
                    koto.run_update(current_time);
                }

                if koto.is_ready() {
                    self.request_animation_frame(ctx);
                    false
                } else {
                    true
                }
            }
            Msg::WindowResized => {
                let canvas = self.get_canvas();
                canvas.set_width(canvas.client_width() as u32);
                canvas.set_height(canvas.client_height() as u32);
                self.get_koto().on_resize();
                false
            }
            Msg::BeforeUnload => {
                set_local_storage_value("script", &self.script);
                set_local_storage_value(
                    "vim-bindings-enabled",
                    if self.vim_bindings_enabled {
                        "true"
                    } else {
                        "false"
                    },
                );
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let editor_area = html! {
            <div class="editor-area">
                <EditorToolbar
                    script_playing={self.run_script_enabled}
                    vim_bindings_enabled={self.vim_bindings_enabled}
                    on_play_clicked={ctx.link().callback(|_| Msg::PlayButtonClicked)}
                    on_reload_clicked={ctx.link().callback(|_| Msg::ReloadButtonClicked)}
                    on_vim_bindings_clicked={ctx.link().callback(|_| Msg::ToggleVimBindings)}
                    on_share_clicked={ctx.link().callback(|_| Msg::ShareButtonClicked)}
                    on_script_selected={
                        ctx.link().callback(|script| Msg::ScriptMenuChanged {script})
                    }
                />

                <Editor
                    on_initialized={ctx.link().callback(|editor| Msg::EditorInitialized {editor})}
                    on_changed={ctx.link().callback(|_| Msg::EditorChanged)}
                />
            </div>
        };

        html! {
            <div class="playground">
                { editor_area }

                <canvas
                  ref={self.canvas_ref.clone()}
                  id="koto-canvas"
                  class="fullsize"
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
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.get_canvas();

            canvas.set_width(canvas.client_width() as u32);
            canvas.set_height(canvas.client_height() as u32);

            let compiler_output = self.compiler_output_ref.cast::<Element>().unwrap();
            let script_output = self.script_output_ref.cast::<Element>().unwrap();

            self.koto = Some(KotoWrapper::new(canvas, compiler_output, script_output));
        }
    }
}
