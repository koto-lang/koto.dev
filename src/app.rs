use {
    crate::{get_ace, get_element_by_id, koto_wrapper::KotoWrapper, APP},
    gloo_events::EventListener,
    gloo_render::AnimationFrame,
    gloo_utils::window,
    wasm_bindgen::JsCast,
    web_sys::HtmlCanvasElement,
};

pub struct App {
    koto: KotoWrapper,
    canvas: HtmlCanvasElement,
    last_time: Option<f64>,
    current_time: f64,
    animation_frame: Option<AnimationFrame>,
}

impl App {
    pub fn new() -> Self {
        let canvas = get_element_by_id("koto-canvas");
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        EventListener::new(&window(), "resize", |_| {
            APP.with(|app| app.borrow_mut().on_window_resize());
        })
        .forget();

        Self {
            koto: KotoWrapper::new(canvas.clone()),
            canvas,
            last_time: None,
            current_time: 0.0,
            animation_frame: None,
        }
    }

    pub fn reset(&mut self) {
        self.koto.reset();
        self.animation_frame = None;
        self.last_time = None;
    }

    pub fn on_script_edited(&mut self) {
        let script = get_ace().edit("editor").get_session().get_value();
        if !script.is_empty() {
            self.koto.compile_script(&script);

            if self.koto.update_should_be_called() {
                self.request_animation_frame();
            }
        }
    }

    fn request_animation_frame(&mut self) {
        self.animation_frame = Some(gloo_render::request_animation_frame(
            Self::on_animation_frame_handler,
        ));
    }

    fn on_animation_frame_handler(time: f64) {
        APP.with(|app| app.borrow_mut().on_animation_frame(time));
    }

    fn on_animation_frame(&mut self, time: f64) {
        let time_delta = (time - self.last_time.unwrap_or(time)) / 1000.0;
        self.current_time += time_delta;
        self.last_time = Some(time);

        if self.koto.is_ready() {
            self.koto.run_update(self.current_time);
        }

        if self.koto.is_ready() {
            self.request_animation_frame();
        }
    }

    pub fn on_window_resize(&mut self) {
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);
    }
}
