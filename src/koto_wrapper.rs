use {
    cloned::cloned,
    koto::{
        runtime::{
            unexpected_type_error_with_slice, CallArgs, KotoFile, KotoRead, KotoWrite,
            RuntimeError, Value, ValueMap,
        },
        Koto, KotoError, KotoSettings,
    },
    std::{cell::RefCell, collections::VecDeque, fmt, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement},
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
    Clear,
    ClearOutput,
    Fill,
    FillRect(Rect),
    LineTo(Point),
    MoveTo(Point),
    Print(String),
    Rect(Rect),
    Rotate(f64),
    SetFillColor(Color),
    SetLineWidth(f64),
    SetStrokeColor(Color),
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

pub enum ScriptState {
    NotReady,
    Compiled,
    Recompiled,
    Initialized,
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
}

impl KotoWrapper {
    pub fn new(
        canvas: HtmlCanvasElement,
        compiler_output: Element,
        script_output: Element,
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
        !matches!(self.script_state, ScriptState::NotReady)
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self.script_state, ScriptState::Initialized)
    }

    pub fn update_should_be_called(&self) -> bool {
        self.is_initialized() && self.play_module.data().get_with_string("update").is_some()
    }

    fn log_error(&mut self, error: &str) {
        self.script_state = ScriptState::NotReady;
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
            None => Ok(Value::Empty),
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
                KotoMessage::Clear => {
                    self.canvas_context.clear_rect(
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
                KotoMessage::SetLineWidth(width) => self.canvas_context.set_line_width(width),
                KotoMessage::SetStrokeColor(color) => {
                    let color_rgb = color.as_css_rgb();
                    self.canvas_context
                        .set_stroke_style(&JsValue::from(color_rgb))
                }
                KotoMessage::SetTransform { a, b, c, d, e, f } => {
                    self.canvas_context.set_transform(a, b, c, d, e, f).unwrap()
                }
                KotoMessage::Stroke => self.canvas_context.stroke(),
                KotoMessage::StrokeRect(r) => {
                    self.canvas_context.stroke_rect(r.x, r.y, r.width, r.height)
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
            Ok(Empty)
        }
    });

    result
}

fn make_canvas_module(canvas: HtmlCanvasElement, queue: KotoMessageQueue) -> ValueMap {
    use Value::*;

    let result = ValueMap::default();

    result.add_fn("arc", {
        cloned!(queue);
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
            Ok(Empty)
    }});

    result.add_fn("begin_path", {
        cloned!(queue);
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::BeginPath);
            Ok(Empty)
        }
    });

    result.add_fn("clear", {
        cloned!(queue);
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::Clear);
            Ok(Empty)
        }
    });

    result.add_fn("fill", {
        cloned!(queue);
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::Fill);
            Ok(Empty)
        }
    });

    result.add_fn("fill_rect", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("height", {
        cloned!(canvas);
        move |_, _| Ok(Number(canvas.height().into()))
    });

    result.add_fn("line_to", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("move_to", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("rect", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("rotate", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("set_fill_color", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("set_line_width", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("set_stroke_color", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("set_transform", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("stroke", {
        cloned!(queue);
        move |_, _| {
            queue.borrow_mut().push_back(KotoMessage::Stroke);
            Ok(Empty)
        }
    });

    result.add_fn("stroke_rect", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("translate", {
        cloned!(queue);
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
            Ok(Empty)
        }
    });

    result.add_fn("width", {
        cloned!(canvas);
        move |_, _| Ok(Number(canvas.width().into()))
    });

    result
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
