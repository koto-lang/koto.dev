use {
    cloned::cloned,
    koto::{
        runtime::{
            runtime_error, unexpected_type_error_with_slice, CallArgs, KotoFile, KotoRead,
            KotoWrite, RuntimeError, Value, ValueMap,
        },
        Koto, KotoError, KotoSettings,
    },
    rand::{thread_rng, Rng},
    std::{cell::RefCell, collections::VecDeque, fmt, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement},
    yew::Callback,
};

pub type KotoMessageQueue = Rc<RefCell<VecDeque<KotoMessage>>>;

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
    Stroke,
    StrokeRect(Rect),
    StrokeText {
        text: String,
        position: Point,
        max_width: Option<f64>,
    },
    Translate(Point),
}

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Color {
    fn as_css_rgb(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
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
}

impl KotoWrapper {
    pub fn new(
        canvas: HtmlCanvasElement,
        compiler_output: Element,
        script_output: Element,
        on_fps_changed: Callback<f64>,
    ) -> Self {
        let message_queue = KotoMessageQueue::default();

        let koto = Koto::with_settings(
            KotoSettings::default()
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
        }
    }

    pub fn compile_script(&mut self, script: &str) {
        debug_assert!(!script.is_empty());

        self.message_queue.borrow_mut().clear();

        {
            let mut play_module = self.play_module.data_mut();
            play_module.remove_with_string("setup");
            play_module.remove_with_string("on_load");
            play_module.remove_with_string("update");
        }

        self.koto.clear_module_cache();
        if let Err(error) = self.koto.compile(&script) {
            self.log_error(&format!("Error while compiling script: {error}"));
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
            return self.log_error(&e.to_string());
        }

        if matches!(self.script_state, ScriptState::Compiled) {
            let maybe_fn = self.play_module.data().get_with_string("setup").cloned();
            self.user_state = match maybe_fn {
                Some(f) => match self.koto.run_function(f, CallArgs::None) {
                    Ok(state) => state,
                    Err(e) => {
                        return self.log_error(&e.to_string());
                    }
                },
                None => Value::Map(ValueMap::default()),
            };
        }

        if let Err(e) = self.run_play_function("on_load", &[self.user_state.clone()]) {
            return self.log_error(&e.to_string());
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
        self.is_initialized() && self.play_module.data().get_with_string("update").is_some()
    }

    fn log_error(&mut self, error: &str) {
        use ScriptState::*;
        self.script_state = match self.script_state {
            Initialized | ErrorAfterInitialized => ErrorAfterInitialized,
            _ => NotReady,
        };

        self.compiler_output.set_inner_html(error);
        self.compiler_output
            .set_scroll_top(self.compiler_output.scroll_height());
    }

    fn run_play_function(
        &mut self,
        function_name: &str,
        args: &[Value],
    ) -> Result<Value, KotoError> {
        match self.play_module.data().get_with_string(function_name) {
            Some(f) => self.koto.run_function(f.clone(), CallArgs::Separate(args)),
            None => Ok(Value::Null),
        }
    }

    pub fn on_resize(&mut self) {
        if self.is_ready() {
            if let Err(e) = self.koto.run() {
                return self.log_error(&e.to_string());
            }

            self.process_koto_messages();
        }
    }

    pub fn run_update(&mut self, time: f64) {
        debug_assert!(self.is_ready());

        match self.run_play_function("update", &[self.user_state.clone(), time.into()]) {
            Ok(_) => {
                self.process_koto_messages();
            }
            Err(e) => {
                self.log_error(&e.to_string());
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
                    let color_rgb = color.as_css_rgb();
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
                    self.canvas_context.fill_rect(r.x, r.y, r.width, r.height)
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
                KotoMessage::Rect(r) => self.canvas_context.rect(r.x, r.y, r.width, r.height),
                KotoMessage::Rotate(radians) => self.canvas_context.rotate(radians).unwrap(),
                KotoMessage::SetFillColor(color) => {
                    let color_rgb = color.as_css_rgb();
                    self.canvas_context
                        .set_fill_style(&JsValue::from(color_rgb))
                }
                KotoMessage::SetFont(font) => self.canvas_context.set_font(&font),
                KotoMessage::SetFps(fps) => self.on_fps_changed.emit(fps),
                KotoMessage::SetLineWidth(width) => self.canvas_context.set_line_width(width),
                KotoMessage::SetStrokeColor(color) => {
                    let color_rgb = color.as_css_rgb();
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
                KotoMessage::Stroke => self.canvas_context.stroke(),
                KotoMessage::StrokeRect(r) => {
                    self.canvas_context.stroke_rect(r.x, r.y, r.width, r.height)
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
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::ClearOutput);
            Ok(Null)
        }
    });

    result.add_fn("random_color", {
        move |vm, args| {
            let alpha = match vm.get_args(args) {
                [] => 1.0,
                [Number(alpha)] => alpha.into(),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "play.random_color",
                        "an optional alpha value",
                        unexpected,
                    )
                }
            };

            let mut rng = thread_rng();
            let r: u8 = rng.gen_range(0..=255);
            let g: u8 = rng.gen_range(0..=255);
            let b: u8 = rng.gen_range(0..=255);
            Ok(Num4(koto::runtime::num4::Num4(
                r.into(),
                g.into(),
                b.into(),
                alpha,
            )))
        }
    });

    result.add_fn("set_fps", {
        cloned!(queue);
        move |vm, args| {
            let fps = match vm.get_args(args) {
                [Number(fps)] if *fps >= 0.0 => f64::from(fps),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "play.set_fps",
                        "a non-negative Number",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::SetFps(fps));
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
        move |vm, args| {
            let (x, y, radius, start_angle, end_angle, counter_clockwise) = match vm.get_args(args) {
                [Num2(n), Number(radius), Number(start_angle), Number(end_angle)] => {
                    (n[0], n[1], radius.into(), start_angle.into(), end_angle.into(), false)
                },
                [Num2(n), Number(radius), Number(start_angle), Number(end_angle), Bool(counter_clockwise)] => {
                    (n[0], n[1], radius.into(), start_angle.into(), end_angle.into(), *counter_clockwise)
                }
                [Number(x), Number(y), Number(radius), Number(start_angle), Number(end_angle)] => {
                    (x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), false)
                }
                [Number(x), Number(y), Number(radius), Number(start_angle), Number(end_angle), Bool(counter_clockwise)] => {
                    (x.into(), y.into(), radius.into(), start_angle.into(), end_angle.into(), *counter_clockwise)
                }
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.arc",
                        "x & y (as Numbers or a Num2), radius, start_angle, end_angle, counter_clockwise (optional)",
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
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::BeginPath);
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("clear", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let maybe_color = match vm.get_args(args) {
                [] => None,
                [Number(n1), Number(n2), Number(n3)] => {
                    Some((n1.into(), n2.into(), n3.into(), 1.0))
                }
                [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                    Some((n1.into(), n2.into(), n3.into(), n4.into()))
                }
                [Num4(color)] => Some((
                    color.0 as f64,
                    color.1 as f64,
                    color.2 as f64,
                    color.3 as f64,
                )),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "play.random_color",
                        "an optional alpha value",
                        unexpected,
                    )
                }
            }
            .map(|(r, g, b, a)| Color { r, g, b, a });

            queue
                .borrow_mut()
                .push_back(KotoMessage::Clear(maybe_color));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("fill", {
        cloned!(canvas_module, queue);
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::Fill);
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("fill_rect", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (x, y, width, height) = match vm.get_args(args) {
                [Num2(pos), Number(width), Number(height)] => {
                    (pos[0], pos[1], width.into(), height.into())
                }
                [Number(x), Number(y), Number(width), Number(height)] => {
                    (x.into(), y.into(), width.into(), height.into())
                }
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.fill_rect",
                        "(x and y (as Numbers or a Num2), width, height)",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::FillRect(Rect {
                x,
                y,
                width,
                height,
            }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("fill_text", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (text, x, y, max_width) = match vm.get_args(args) {
                [Str(text), Number(x), Number(y)] => (text, x.into(), y.into(), None),
                [Str(text), Number(x), Number(y), Number(max_width)] => {
                    (text, x.into(), y.into(), Some(max_width.into()))
                }
                [Str(text), Num2(n)] => (text, n.0, n.1, None),
                [Str(text), Num2(n), Number(max_width)] => (text, n.0, n.1, Some(max_width.into())),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.fill_text",
                        "(text, x and y (as Numbers or a Num2), max width (optional))",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::FillText {
                text: text.to_string(),
                position: Point { x, y },
                max_width,
            });
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("height", {
        cloned!(canvas);
        move |_, _| Ok(Number(canvas.height().into()))
    });

    canvas_module.add_fn("line_to", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (x, y) = match vm.get_args(args) {
                [Number(x), Number(y)] => (x.into(), y.into()),
                [Num2(n)] => (n[0], n[1]),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.line_to",
                        "two Numbers or a Num2",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::LineTo(Point { x, y }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("move_to", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (x, y) = match vm.get_args(args) {
                [Number(x), Number(y)] => (x.into(), y.into()),
                [Num2(n)] => (n[0], n[1]),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.move_to",
                        "two Numbers or a Num2",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::MoveTo(Point { x, y }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("rect", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (x, y, width, height) = match vm.get_args(args) {
                [Num2(pos), Number(width), Number(height)] => {
                    (pos[0], pos[1], width.into(), height.into())
                }
                [Number(x), Number(y), Number(width), Number(height)] => {
                    (x.into(), y.into(), width.into(), height.into())
                }
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.rect",
                        "x and y (as Numbers or a Num2), width, height",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::Rect(Rect {
                x,
                y,
                width,
                height,
            }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("rotate", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let n = match vm.get_args(args) {
                [Number(n)] => n.into(),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.rotate",
                        "a Number in radians",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::Rotate(n));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_fill_color", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (r, g, b, a) = match vm.get_args(args) {
                [Number(n1), Number(n2), Number(n3)] => (n1.into(), n2.into(), n3.into(), 1.0),
                [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                    (n1.into(), n2.into(), n3.into(), n4.into())
                }
                [Num4(color)] => (
                    color.0 as f64,
                    color.1 as f64,
                    color.2 as f64,
                    color.3 as f64,
                ),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.set_fill_color",
                        "3 or 4 Numbers or a Num4",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetFillColor(Color { r, b, g, a }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_font", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let font = match vm.get_args(args) {
                [Str(font)] => font,
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.set_font",
                        "a String",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetFont(font.to_string()));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_line_width", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let width = match vm.get_args(args) {
                [Number(n)] => n,
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.set_line_width",
                        "a Number",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetLineWidth(width.into()));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_stroke_color", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (r, g, b, a) = match vm.get_args(args) {
                [Number(n1), Number(n2), Number(n3)] => (n1.into(), n2.into(), n3.into(), 1.0),
                [Number(n1), Number(n2), Number(n3), Number(n4)] => {
                    (n1.into(), n2.into(), n3.into(), n4.into())
                }
                [Num4(color)] => (
                    color.0 as f64,
                    color.1 as f64,
                    color.2 as f64,
                    color.3 as f64,
                ),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.set_stroke_color",
                        "3 or 4 Numbers or a Num4",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetStrokeColor(Color { r, b, g, a }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("set_text_align", {
        cloned!(canvas_module, queue);
        let allowed_values = &["left", "right", "center", "start", "end"];
        move |vm, args| {
            let text_align = match vm.get_args(args) {
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
                    return unexpected_type_error_with_slice(
                        "canvas.set_text_align",
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
        move |vm, args| {
            let baseline = match vm.get_args(args) {
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
                    return unexpected_type_error_with_slice(
                        "canvas.set_text_baseline",
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
        move |vm, args| {
            let (a, b, c, d, e, f) = match vm.get_args(args) {
                [Number(a), Number(b), Number(c), Number(d), Number(e), Number(f)] => {
                    (a.into(), b.into(), c.into(), d.into(), e.into(), f.into())
                }
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.set_transform",
                        "6 Numbers",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::SetTransform { a, b, c, d, e, f });
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("stroke", {
        cloned!(canvas_module, queue);
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::Stroke);
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("stroke_rect", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (x, y, width, height) = match vm.get_args(args) {
                [Num2(pos), Number(width), Number(height)] => {
                    (pos[0], pos[1], width.into(), height.into())
                }
                [Number(x), Number(y), Number(width), Number(height)] => {
                    (x.into(), y.into(), width.into(), height.into())
                }
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.stroke_rect",
                        "(x and y (as Numbers or a Num2), width, height)",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::StrokeRect(Rect {
                x,
                y,
                width,
                height,
            }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("stroke_text", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (text, x, y, max_width) = match vm.get_args(args) {
                [Str(text), Number(x), Number(y)] => (text, x.into(), y.into(), None),
                [Str(text), Number(x), Number(y), Number(max_width)] => {
                    (text, x.into(), y.into(), Some(max_width.into()))
                }
                [Str(text), Num2(n)] => (text, n.0, n.1, None),
                [Str(text), Num2(n), Number(max_width)] => (text, n.0, n.1, Some(max_width.into())),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.stroke_text",
                        "(text, x and y (as Numbers or a Num2), max width (optional))",
                        unexpected,
                    )
                }
            };
            queue.borrow_mut().push_back(KotoMessage::StrokeText {
                text: text.to_string(),
                position: Point { x, y },
                max_width,
            });
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("translate", {
        cloned!(canvas_module, queue);
        move |vm, args| {
            let (x, y) = match vm.get_args(args) {
                [Number(x), Number(y)] => (x.into(), y.into()),
                [Num2(n)] => (n[0], n[1]),
                unexpected => {
                    return unexpected_type_error_with_slice(
                        "canvas.translate",
                        "two Numbers or a Num2",
                        unexpected,
                    )
                }
            };
            queue
                .borrow_mut()
                .push_back(KotoMessage::Translate(Point { x, y }));
            Ok(Map(canvas_module.clone()))
        }
    });

    canvas_module.add_fn("width", {
        cloned!(canvas);
        move |_, _| Ok(Number(canvas.width().into()))
    });

    canvas_module
}

// Captures output from Koto in a String
struct OutputCapture {
    id: String,
    queue: KotoMessageQueue,
}

impl KotoFile for OutputCapture {}
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

impl fmt::Display for OutputCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

impl fmt::Debug for OutputCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}
