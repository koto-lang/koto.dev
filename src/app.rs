use {
    crate::{get_element_by_id, koto_wrapper::KotoWrapper, APP},
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
    _window_resize_listener: EventListener,
    animation_frame: Option<AnimationFrame>,
}

impl App {
    pub fn new() -> Self {
        let canvas = get_element_by_id("koto-canvas");
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .expect("koto-canvas is the wrong element type");

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let window_resize_listener = EventListener::new(&window(), "resize", |_| {
            APP.with(|app| app.borrow_mut().on_window_resize());
        });

        Self {
            koto: KotoWrapper::new(canvas.clone()),
            canvas,
            last_time: None,
            _window_resize_listener: window_resize_listener,
            animation_frame: None,
        }
    }

    pub fn compile_script_and_call_setup(&mut self) {
        self.koto.compile_script(true);

        if self.koto.update_should_be_called() {
            self.request_animation_frame();
        }
    }

    pub fn on_script_changed(&mut self) {
        self.koto.compile_script(false);

        if self.koto.update_should_be_called() {
            self.request_animation_frame();
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
        let time_delta = if let Some(last_time) = self.last_time {
            (time - last_time) / 1000.0
        } else {
            0.0
        };

        self.last_time = Some(time);

        self.koto.run_update(time_delta);

        if self.koto.is_ready() {
            self.request_animation_frame();
        }
    }

    fn on_window_resize(&mut self) {
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);
    }
}
