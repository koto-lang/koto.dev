use {
    super::{editor::Editor, editor_toolbar::EditorToolbar},
    crate::{
        ace_bindings::AceEditor, get_local_storage_value, koto_wrapper::KotoWrapper,
        set_local_storage_value,
    },
    gloo_events::EventListener,
    gloo_render::AnimationFrame,
    gloo_utils::window,
    web_sys::HtmlCanvasElement,
    yew::prelude::*,
};

pub enum Msg {
    EditorInitialized { editor: AceEditor },
    EditorChanged { script: Box<str> },
    ScriptMenuChanged { script: &'static str },
    ToggleVimBindings,
    AnimationFrame { time: f64 },
    WindowResized,
    BeforeUnload,
}

pub struct Playground {
    self_ref: NodeRef,
    canvas_ref: NodeRef,
    editor: Option<AceEditor>,
    koto: Option<KotoWrapper>,

    script: Box<str>,
    vim_bindings_enabled: bool,

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

    fn set_editor_contents(&mut self, contents: &str) {
        self.get_editor().get_session().set_value(contents);
    }

    fn reset(&mut self) {
        self.get_koto().reset();
        self.animation_frame = None;
        self.last_time = None;
    }

    fn set_vim_bindings_enabled(&mut self, enabled: bool) {
        self.vim_bindings_enabled = enabled;
        self.get_editor()
            .set_keyboard_handler(if enabled { "ace/keyboard/vim" } else { "" });
    }
}

impl Component for Playground {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            self_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            editor: None,
            koto: None,
            script: "".into(),
            vim_bindings_enabled: get_local_storage_value("vim-bindings-enabled")
                .map_or(false, |enabled| enabled == "true"),
            animation_frame: None,
            last_time: None,
            current_time: 0.0,
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
                self.set_editor_contents(
                    get_local_storage_value("script")
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or(include_str!("../scripts/canvas/random_rects.koto")),
                );
                self.set_vim_bindings_enabled(self.vim_bindings_enabled);
                false
            }
            Msg::EditorChanged { script } => {
                self.animation_frame = None;
                if !script.is_empty() {
                    let koto = self.get_koto();
                    koto.compile_script(&script);

                    if koto.update_should_be_called() {
                        self.request_animation_frame(ctx)
                    }
                }
                self.script = script;
                false
            }
            Msg::ScriptMenuChanged { script } => {
                self.reset();
                self.set_editor_contents(script);
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
                    self.request_animation_frame(ctx)
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
                    vim_bindings_enabled={self.vim_bindings_enabled}
                    on_vim_bindings_clicked={ctx.link().callback(|_| Msg::ToggleVimBindings)}
                    on_script_selected={ctx.link().callback(|script| Msg::ScriptMenuChanged {script})}
                />

                <Editor
                    on_changed={ctx.link().callback(|script| Msg::EditorChanged {script})}
                    on_initialized={ctx.link().callback(|editor| Msg::EditorInitialized {editor})}
                />
            </div>
        };

        html! {
            <div
              ref={self.self_ref.clone()}
              class="playground"
            >
                { editor_area }

                <canvas
                  ref={self.canvas_ref.clone()}
                  id="koto-canvas"
                  class="fullsize"
                  width="400"
                  height="400"
                ></canvas>

                <textarea
                  id="compiler-output"
                  class="fixed-mono"
                  readonly=true
                ></textarea>

                <textarea
                  id="script-output"
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

            self.koto = Some(KotoWrapper::new(canvas));
        }
    }
}
