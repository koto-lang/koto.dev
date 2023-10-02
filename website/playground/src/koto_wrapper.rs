use {
    cloned::cloned,
    koto::prelude::*,
    koto_color::Color,
    koto_geometry::{Rect, Vec2},
    rand::{thread_rng, Rng},
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement},
    yew::Callback,
};

pub type KotoMessageQueue = Rc<RefCell<VecDeque<KotoMessage>>>;

type Point = Vec2;

pub enum KotoMessage {
    Arc {
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counter_clockwise: bool,
    },
    BeginPath,
    Clear(Option<Color>),
    ClearOutput,
    Fill,
    FillRect(Rect),
    FillText {
        text: String,
        position: Point,
        max_width: Option<f64>,
    },
    LineTo(Point),
    MoveTo(Point),
    Print(String),
    Rect(Rect),
    Rotate(f64),
    SetFillColor(Color),
    SetFont(String),
    SetFps(f64),
    SetLineWidth(f64),
    SetStrokeColor(Color),
    SetTextAlign(String),
    SetTextBaseline(String),
    SetTransform {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },
    ShowCanvas,
    Stroke,
    StrokeRect(Rect),
    StrokeText {
        text: String,
        position: Point,
        max_width: Option<f64>,
    },
    Translate(Point),
}

fn color_as_css_rgb(c: Color) -> String {
    format!(
        "rgba({}%, {}%, {}%, {})",
        c.red() * 100.0,
        c.green() * 100.0,
        c.blue() * 100.0,
        c.alpha()
    )
}

#[derive(Debug)]
pub enum ScriptState {
    NotReady,
    Compiled,
    Recompiled,
    Initialized,
    ErrorAfterInitialized,
}

pub struct KotoWrapper {
    koto: Koto,
    play_module: ValueMap,
    compiler_output: Element,
    script_output: Element,
    canvas: HtmlCanvasElement,
    canvas_context: CanvasRenderingContext2d,
    output_buffer: String,
    message_queue: KotoMessageQueue,
    script_state: ScriptState,
    user_state: Value,
    on_fps_changed: Callback<f64>,
    on_show_canvas: Callback<()>,
}

impl KotoWrapper {
    pub fn new(
        canvas: HtmlCanvasElement,
        compiler_output: Element,
        script_output: Element,
        on_fps_changed: Callback<f64>,
        on_show_canvas: Callback<()>,
    ) -> Self {
        let message_queue = KotoMessageQueue::default();

        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_stdin(PlaygroundInput {})
                .with_stdout(OutputCapture {
                    id: "_stdout_".into(),
                    queue: message_queue.clone(),
                })
                .with_stderr(OutputCapture {
                    id: "_stderr_".into(),
                    queue: message_queue.clone(),
                }),
        );

        koto.prelude().add_map(
            "canvas",
            make_canvas_module(canvas.clone(), message_queue.clone()),
        );
        let play_module = make_play_module(message_queue.clone());
        koto.prelude().add_map("color", koto_color::make_module());
        koto.prelude()
            .add_map("geometry", koto_geometry::make_module());
        koto.prelude().add_map("play", play_module.clone());
        koto.prelude().add_map("random", koto_random::make_module());

        let canvas_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        canvas_context.set_fill_style(&JsValue::from("#999999"));
        canvas_context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        Self {
            koto,
            play_module,
            compiler_output,
            script_output,
            canvas,
            canvas_context,
            output_buffer: String::with_capacity(128),
            message_queue,
            script_state: ScriptState::NotReady,
            user_state: Value::Map(ValueMap::default()),
            on_fps_changed,
            on_show_canvas,
        }
    }

    pub fn compile_script(&mut self, script: &str) {
        debug_assert!(!script.is_empty());

        self.message_queue.borrow_mut().clear();

        self.koto.exports().data_mut().clear();
        self.koto.clear_module_cache();

        if let Err(error) = self.koto.compile(&script) {
            self.error(&format!("Error while compiling script: {error}"));
            return;
        }

        self.compiler_output.set_inner_html("Success");
        self.script_state = if matches!(self.script_state, ScriptState::NotReady) {
            ScriptState::Compiled
        } else {
            ScriptState::Recompiled
        }
    }

    pub fn run(&mut self) {
        if !self.is_ready() {
            panic!("Attempting to run koto script when not in a ready state");
        }

        if self.is_initialized() {
            panic!("Attempting to run koto script when already initialized");
        }

        if let Err(e) = self.koto.run() {
            return self.error(&e.to_string());
        }

        if matches!(self.script_state, ScriptState::Compiled) {
            self.user_state = match self.run_exported_function("setup", &[]) {
                Ok(Some(data)) => data,
                Ok(None) => ValueMap::default().into(),
                Err(error) => {
                    return self.error(&error.to_string());
                }
            };
        }

        if let Err(e) = self.run_exported_function("on_load", &[self.user_state.clone()]) {
            return self.error(&e.to_string());
        }

        self.script_state = ScriptState::Initialized;

        self.process_koto_messages();
    }

    pub fn reset(&mut self) {
        self.script_state = ScriptState::NotReady;
        self.canvas_context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
        self.compiler_output.set_inner_html("");
        self.script_output.set_inner_html("");
    }

    pub fn is_ready(&self) -> bool {
        !matches!(
            self.script_state,
            ScriptState::NotReady | ScriptState::ErrorAfterInitialized
        )
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self.script_state, ScriptState::Initialized)
    }

    pub fn update_should_be_called(&self) -> bool {
        self.is_initialized() && self.play_module.data().get("update").is_some()
    }

    fn error(&mut self, error: &str) {
        use ScriptState::*;
        self.script_state = match self.script_state {
            Initialized | Recompiled | ErrorAfterInitialized => ErrorAfterInitialized,
            _ => NotReady,
        };

        self.compiler_output.set_inner_html(error);
        self.compiler_output
            .set_scroll_top(self.compiler_output.scroll_height());
    }

    fn run_exported_function(
        &mut self,
        function_name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, koto::Error> {
        match self
            .koto
            .run_exported_function(function_name, CallArgs::Separate(args))
        {
            Ok(result) => Ok(Some(result)),
            Err(koto::Error::FunctionNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn on_resize(&mut self) {
        if self.is_ready() {
            if let Err(e) = self.koto.run() {
                return self.error(&e.to_string());
            }

            self.process_koto_messages();
        }
    }

    pub fn run_update(&mut self, time: f64) {
        debug_assert!(self.is_ready());

        match self.run_exported_function("update", &[self.user_state.clone(), time.into()]) {
            Ok(_) => {
                self.process_koto_messages();
            }
            Err(e) => {
                self.error(&e.to_string());
            }
        }
    }

    fn process_koto_messages(&mut self) {
        for message in self.message_queue.borrow_mut().drain(..) {
            match message {
                KotoMessage::Arc {
                    x,
                    y,
                    radius,
                    start_angle,
                    end_angle,
                    counter_clockwise,
                } => {
                    self.canvas_context
                        .arc_with_anticlockwise(
                            x,
                            y,
                            radius,
                            start_angle,
                            end_angle,
                            counter_clockwise,
                        )
                        .unwrap();
                }
                KotoMessage::BeginPath => self.canvas_context.begin_path(),
                KotoMessage::Clear(None) => {
                    self.canvas_context.clear_rect(
                        0.0,
                        0.0,
                        self.canvas.width() as f64,
                        self.canvas.height() as f64,
                    );
                }
                KotoMessage::Clear(Some(color)) => {
                    let color_rgb = color_as_css_rgb(color);
                    self.canvas_context
                        .set_fill_style(&JsValue::from(color_rgb));
                    self.canvas_context.fill_rect(
                        0.0,
                        0.0,
                        self.canvas.width() as f64,
                        self.canvas.height() as f64,
                    );
                }
                KotoMessage::ClearOutput => self.script_output.set_inner_html(""),
                KotoMessage::Fill => self.canvas_context.fill(),
                KotoMessage::FillRect(r) => {
                    let (x, y, w, h) = r.x_y_w_h();
                    self.canvas_context.fill_rect(x, y, w, h)
                }
                KotoMessage::FillText {
                    text,
                    position,
                    max_width,
                } => {
                    if let Some(max_width) = max_width {
                        self.canvas_context
                            .fill_text_with_max_width(&text, position.x, position.y, max_width)
                            .unwrap()
                    } else {
                        self.canvas_context
                            .fill_text(&text, position.x, position.y)
                            .unwrap();
                    }
                }
                KotoMessage::LineTo(p) => self.canvas_context.line_to(p.x, p.y),
                KotoMessage::MoveTo(p) => self.canvas_context.move_to(p.x, p.y),
                KotoMessage::Print(s) => self.output_buffer.push_str(&s),
                KotoMessage::Rect(r) => {
                    let (x, y, w, h) = r.x_y_w_h();
                    self.canvas_context.rect(x, y, w, h)
                }
                KotoMessage::Rotate(radians) => self.canvas_context.rotate(radians).unwrap(),
                KotoMessage::SetFillColor(color) => {
                    let color_rgb = color_as_css_rgb(color);
                    self.canvas_context
                        .set_fill_style(&JsValue::from(color_rgb))
                }
                KotoMessage::SetFont(font) => self.canvas_context.set_font(&font),
                KotoMessage::SetFps(fps) => self.on_fps_changed.emit(fps),
                KotoMessage::SetLineWidth(width) => self.canvas_context.set_line_width(width),
                KotoMessage::SetStrokeColor(color) => {
                    let color_rgb = color_as_css_rgb(color);
                    self.canvas_context
                        .set_stroke_style(&JsValue::from(color_rgb))
                }
                KotoMessage::SetTransform { a, b, c, d, e, f } => {
                    self.canvas_context.set_transform(a, b, c, d, e, f).unwrap()
                }
                KotoMessage::SetTextAlign(text_align) => {
                    self.canvas_context.set_text_align(&text_align);
                }
                KotoMessage::SetTextBaseline(baseline) => {
                    self.canvas_context.set_text_baseline(&baseline);
                }
                KotoMessage::ShowCanvas => self.on_show_canvas.emit(()),
                KotoMessage::Stroke => self.canvas_context.stroke(),
                KotoMessage::StrokeRect(r) => {
                    let (x, y, w, h) = r.x_y_w_h();
                    self.canvas_context.stroke_rect(x, y, w, h)
                }
                KotoMessage::StrokeText {
                    text,
                    position,
                    max_width,
                } => {
                    if let Some(max_width) = max_width {
                        self.canvas_context
                            .stroke_text_with_max_width(&text, position.x, position.y, max_width)
                            .unwrap()
                    } else {
                        self.canvas_context
                            .stroke_text(&text, position.x, position.y)
                            .unwrap();
                    }
                }
                KotoMessage::Translate(p) => self.canvas_context.translate(p.x, p.y).unwrap(),
            }
        }

        if !self.output_buffer.is_empty() {
            self.script_output
                .append_with_str_1(&self.output_buffer)
                .unwrap();
            self.script_output
                .set_scroll_top(self.script_output.scroll_height());
            self.output_buffer.clear();
        }
    }
}

fn make_play_module(queue: KotoMessageQueue) -> ValueMap {
    use Value::*;

    let result = ValueMap::default();

    result.add_fn("clear_output", {
        cloned!(queue);
        move |_| {
            queue.borrow_mut().push_back(KotoMessage::ClearOutput);
            Ok(Null)
        }
    });

    result.add_fn("random_color", {
        move |ctx| {
            let alpha = match ctx.args() {
                [] => 1.0,
                [Number(alpha)] => alpha.into(),
                unexpected => return type_error_with_slice("an optional alpha value", unexpected),
            };

            let rgb: [f32; 3] = thread_rng().gen();
            Ok(Color::rgba(rgb[0], rgb[1], rgb[2], alpha).into())
        }
    });

    result.add_fn("set_fps", {
        cloned!(queue);
        move |ctx| {
            let fps = match ctx.args() {
                [Number(fps)] if *fps >= 0.0 => f64::from(fps),
                unexpected => return type_error_with_slice("a non-negative Number", unexpected),
            };
            queue.borrow_mut().push_back(KotoMessage::SetFps(fps));
            Ok(Null)
        }
    });

    result.add_fn("show_canvas", {
        cloned!(queue);
        move |_| {
            queue.borrow_mut().push_back(KotoMessage::ShowCanvas);
            Ok(Null)
        }
    });

    result
}

fn make_canvas_module(canvas: HtmlCanvasElement, queue: KotoMessageQueue) -> ValueMap {
    use Value::*;

    let canvas_module = ValueMap::default();

    canvas_module.add_fn("arc", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let (x, y, radius, start_angle, end_angle, counter_clockwise) = match ctx.args() {
                [Object(xy), Number(radius), Number(start_angle), Number(end_angle)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (xy.x, xy.y, radius.into(), start_angle.into(), end_angle.into(), false)
                },
                [Object(xy), Number(radius), Number(start_angle), Number(end_angle), Bool(counter_clockwise)] if xy.is_a::<Vec2>()=> {
                    let xy = xy.cast::<Vec2>()?;
                    (xy.x, xy.y, radius.into(), start_angle.into(), end_angle.into(), *counter_clockwise)
                }
                [Number(x), Number(y), Number(radius), Number(start_angle), Number(end_angle)] => {
                    (x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), false)
                }
                [Number(x), Number(y), Number(radius), Number(start_angle), Number(end_angle), Bool(counter_clockwise)] => {
                    (x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), *counter_clockwise)
                }
                unexpected => {
                    return type_error_with_slice(
                        "x & y (as Numbers or a Vec2), radius, start_angle, end_angle, counter_clockwise (optional)",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::Arc {
                x,
                y,
                radius,
                start_angle,
                end_angle,
                counter_clockwise,
            });
            Ok(Map(canvas_module.clone()))
    }});

    canvas_module.add_fn("begin_path", {
        cloned!(canvas_module, queue);
        move |_| {
            queue.borrow_mut().push_back(KotoMessage::BeginPath);
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("clear", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let maybe_color = match ctx.args() {
                [] => None,
                [Number(n1), Number(n2), Number(n3)] => {
                    Some((n1.into(), n2.into(), n3.into(), 1.0))
                }
                [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                    Some((n1.into(), n2.into(), n3.into(), n4.into()))
                }
                [Object(color)] if color.is_a::<Color>() => {
                    let color = color.cast::<Color>()?;
                    Some((color.red(), color.green(), color.blue(), color.alpha()))
                }
                unexpected => return type_error_with_slice("an optional alpha value", unexpected),
            }
            .map(|rgba| Color::from(rgba));

            queue
                .borrow_mut()
                .push_back(KotoMessage::Clear(maybe_color));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("fill", {
        cloned!(canvas_module, queue);
        move |_| {
            queue.borrow_mut().push_back(KotoMessage::Fill);
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("fill_rect", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let rect = match ctx.args() {
                [Object(rect)] if rect.is_a::<Rect>() => rect.cast::<Rect>()?.clone(),
                [Object(xy), Number(width), Number(height)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (xy.x, xy.y, width.into(), height.into()).into()
                }
                [Number(x), Number(y), Number(width), Number(height)] => {
                    (x.into(), y.into(), width.into(), height.into()).into()
                }
                unexpected => {
                    return type_error_with_slice(
                        "a Rect, or (x and y (as Numbers or a Vec2), width, height)",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::FillRect(rect));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("fill_text", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let (text, x, y, max_width) = match ctx.args() {
                [Str(text), Number(x), Number(y)] => (text, x.into(), y.into(), None),
                [Str(text), Number(x), Number(y), Number(max_width)] => {
                    (text, x.into(), y.into(), Some(max_width.into()))
                }
                [Str(text), Object(xy)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (text, xy.x, xy.y, None)
                }
                [Str(text), Object(xy), Number(max_width)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (text, xy.x, xy.y, Some(max_width.into()))
                }
                unexpected => {
                    return type_error_with_slice(
                        "(text, x and y (as Numbers or a Vec2), max width (optional))",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::FillText {
                text: text.to_string(),
                position: (x, y).into(),
                max_width,
            });
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("height", {
        cloned!(canvas);
        move |_| Ok(Number(canvas.height().into()))
    });

    canvas_module.add_fn("line_to", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let xy = match ctx.args() {
                [Object(xy)] if xy.is_a::<Vec2>() => xy.cast::<Vec2>()?.clone(),
                [Number(x), Number(y)] => (x.into(), y.into()).into(),
                unexpected => return type_error_with_slice("two Numbers or a Vec2", unexpected),
            };
            queue.borrow_mut().push_back(KotoMessage::LineTo(xy));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("move_to", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let xy = match ctx.args() {
                [Object(xy)] if xy.is_a::<Vec2>() => xy.cast::<Vec2>()?.clone(),
                [Number(x), Number(y)] => (x.into(), y.into()).into(),
                unexpected => return type_error_with_slice("two Numbers or a Vec2", unexpected),
            };
            queue.borrow_mut().push_back(KotoMessage::MoveTo(xy));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("rect", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let rect = match ctx.args() {
                [Object(rect)] if rect.is_a::<Rect>() => rect.cast::<Rect>()?.clone(),
                [Object(xy), Number(width), Number(height)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (xy.x, xy.y, width.into(), height.into()).into()
                }
                [Number(x), Number(y), Number(width), Number(height)] => {
                    (x.into(), y.into(), width.into(), height.into()).into()
                }
                unexpected => {
                    return type_error_with_slice(
                        "x and y (as Numbers or a Vec2), width, height",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::Rect(rect));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("rotate", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let n = match ctx.args() {
                [Number(n)] => n.into(),
                unexpected => return type_error_with_slice("a Number in radians", unexpected),
            };
            queue.borrow_mut().push_back(KotoMessage::Rotate(n));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_fill_color", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let color = match ctx.args() {
                [Object(color)] if color.is_a::<Color>() => color.cast::<Color>()?.clone(),
                [Number(n1), Number(n2), Number(n3)] => {
                    (n1.into(), n2.into(), n3.into(), 1.0).into()
                }
                [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                    (n1.into(), n2.into(), n3.into(), n4.into()).into()
                }
                unexpected => {
                    return type_error_with_slice("3 or 4 Numbers or a Color", unexpected)
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetFillColor(color));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_font", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let font = match ctx.args() {
                [Str(font)] => font,
                unexpected => return type_error_with_slice("a String", unexpected),
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetFont(font.to_string()));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_line_width", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let width = match ctx.args() {
                [Number(n)] => n,
                unexpected => return type_error_with_slice("a Number", unexpected),
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetLineWidth(width.into()));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_stroke_color", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let color = match ctx.args() {
                [Number(n1), Number(n2), Number(n3)] => {
                    Color::rgba(n1.into(), n2.into(), n3.into(), 1.0)
                }
                [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                    Color::rgba(n1.into(), n2.into(), n3.into(), n4.into())
                }
                [Object(color)] if color.is_a::<Color>() => color.cast::<Color>()?.clone(),
                unexpected => {
                    return type_error_with_slice("3 or 4 Numbers or a Color", unexpected)
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetStrokeColor(color));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_text_align", {
        cloned!(canvas_module, queue);
        let allowed_values = &["left", "right", "center", "start", "end"];
        move |ctx| {
            let text_align = match ctx.args() {
                [Str(s)] => {
                    if !allowed_values.contains(&s.as_str()) {
                        return runtime_error!(
                            "The provided value must be one of the following: {:?}",
                            allowed_values
                        );
                    }
                    s.to_string()
                }
                unexpected => {
                    return type_error_with_slice(
                        &format!(
                            "a String matching one of the following values: {:?}",
                            allowed_values,
                        ),
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetTextAlign(text_align));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_text_baseline", {
        cloned!(canvas_module, queue);
        let allowed_values = &[
            "top",
            "hanging",
            "middle",
            "alphabetic",
            "ideographic",
            "bottom",
        ];
        move |ctx| {
            let baseline = match ctx.args() {
                [Str(s)] => {
                    if !allowed_values.contains(&s.as_str()) {
                        return runtime_error!(
                            "The provided value must be one of the following: {:?}",
                            allowed_values
                        );
                    }
                    s.to_string()
                }
                unexpected => {
                    return type_error_with_slice(
                        &format!(
                            "a String matching one of the following values: {:?}",
                            allowed_values,
                        ),
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetTextBaseline(baseline));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_transform", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let (a, b, c, d, e, f) = match ctx.args() {
                [Number(a), Number(b), Number(c), Number(d), Number(e), Number(f)] => {
                    (a.into(), b.into(), c.into(), d.into(), e.into(), f.into())
                }
                unexpected => return type_error_with_slice("6 Numbers", unexpected),
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetTransform { a, b, c, d, e, f });
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("stroke", {
        cloned!(canvas_module, queue);
        move |_| {
            queue.borrow_mut().push_back(KotoMessage::Stroke);
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("stroke_rect", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let rect = match ctx.args() {
                [Object(rect)] if rect.is_a::<Rect>() => rect.cast::<Rect>()?.clone(),
                [Object(xy), Number(width), Number(height)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (xy.x, xy.y, width.into(), height.into()).into()
                }
                [Number(x), Number(y), Number(width), Number(height)] => {
                    (x.into(), y.into(), width.into(), height.into()).into()
                }
                unexpected => {
                    return type_error_with_slice(
                        "(x and y (as Numbers or a Vec2), width, height)",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::StrokeRect(rect));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("stroke_text", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let (text, x, y, max_width) = match ctx.args() {
                [Str(text), Number(x), Number(y)] => (text, x.into(), y.into(), None),
                [Str(text), Number(x), Number(y), Number(max_width)] => {
                    (text, x.into(), y.into(), Some(max_width.into()))
                }
                [Str(text), Object(xy)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (text, xy.x, xy.y, None)
                }
                [Str(text), Object(xy), Number(max_width)] if xy.is_a::<Vec2>() => {
                    let xy = xy.cast::<Vec2>()?;
                    (text, xy.x, xy.y, Some(max_width.into()))
                }
                unexpected => {
                    return type_error_with_slice(
                        "(text, x and y (as Numbers or a Vec2), max width (optional))",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::StrokeText {
                text: text.to_string(),
                position: (x, y).into(),
                max_width,
            });
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("translate", {
        cloned!(canvas_module, queue);
        move |ctx| {
            let xy = match ctx.args() {
                [Object(xy)] if xy.is_a::<Vec2>() => xy.cast::<Vec2>()?.clone(),
                [Number(x), Number(y)] => (x.into(), y.into()).into(),
                unexpected => return type_error_with_slice("two Numbers or a Vec2", unexpected),
            };
            queue.borrow_mut().push_back(KotoMessage::Translate(xy));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("width", {
        cloned!(canvas);
        move |_| Ok(Number(canvas.width().into()))
    });

    canvas_module
}

// Shows a prompt when input is requested
struct PlaygroundInput {}

impl KotoFile for PlaygroundInput {
    fn id(&self) -> ValueString {
        "PlaygroundInput".into()
    }
}

impl KotoWrite for PlaygroundInput {}
impl KotoRead for PlaygroundInput {
    fn read_line(&self) -> Result<Option<String>, RuntimeError> {
        runtime_error!("stdin is unsupported in the browser")
    }

    fn read_to_string(&self) -> Result<String, RuntimeError> {
        runtime_error!("stdin is unsupported in the browser")
    }
}

// Captures output from Koto in a String
struct OutputCapture {
    id: ValueString,
    queue: KotoMessageQueue,
}

impl KotoFile for OutputCapture {
    fn id(&self) -> ValueString {
        self.id.clone()
    }
}

impl KotoRead for OutputCapture {}
impl KotoWrite for OutputCapture {
    fn write(&self, bytes: &[u8]) -> Result<(), RuntimeError> {
        let bytes_str = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string().into()),
        };
        self.queue
            .borrow_mut()
            .push_back(KotoMessage::Print(bytes_str.to_string()));
        Ok(())
    }

    fn write_line(&self, output: &str) -> Result<(), RuntimeError> {
        self.queue
            .borrow_mut()
            .push_back(KotoMessage::Print(format!("{output}\n")));
        Ok(())
    }

    fn flush(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}
