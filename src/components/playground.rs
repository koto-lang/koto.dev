use {
    super::editor::Editor,
    crate::{koto_wrapper::KotoWrapper, set_local_storage_value},
    gloo_console::log,
    gloo_events::EventListener,
    gloo_render::AnimationFrame,
    gloo_utils::window,
    web_sys::HtmlCanvasElement,
    yew::prelude::*,
};

pub enum Msg {
    EditorChanged { script: Box<str> },
    AnimationFrame { time: f64 },
    WindowResized,
    BeforeUnload,
}

pub struct Playground {
    self_ref: NodeRef,
    canvas_ref: NodeRef,
    koto: Option<KotoWrapper>,
    animation_frame: Option<AnimationFrame>,

    last_time: Option<f64>,
    current_time: f64,

    window_resized_listener: EventListener,
    before_unload_listener: EventListener,
    script: Box<str>,
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
}

impl Component for Playground {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            self_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            koto: None,
            animation_frame: None,
            last_time: None,
            current_time: 0.0,
            window_resized_listener: EventListener::new(&window(), "resize", {
                let link = ctx.link().clone();
                move |_| link.send_message(Msg::WindowResized)
            }),
            before_unload_listener: EventListener::new(&window(), "beforeunload", {
                let link = ctx.link().clone();
                move |_| link.send_message(Msg::BeforeUnload)
            }),
            script: "".into(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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

                false
            }
            Msg::BeforeUnload => {
                set_local_storage_value("script", &self.script);
                // set_local_storage_value(
                //     "vim-bindings-enabled",
                //     if app.vim_bindings_enabled {
                //         "true"
                //     } else {
                //         "false"
                //     },
                // );

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
              ref={self.self_ref.clone()}
              class="playground"
            >
                <Editor
                  on_changed={ctx.link().callback(|script| Msg::EditorChanged {script})}
                />

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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.get_canvas();

            canvas.set_width(canvas.client_width() as u32);
            canvas.set_height(canvas.client_height() as u32);

            self.koto = Some(KotoWrapper::new(canvas));
        }
    }
}
