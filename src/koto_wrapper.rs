use {
    crate::{get_ace, get_element_by_id, KOTO_MESSAGE_QUEUE},
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
    Fill,
    MoveTo {
        x: f64,
        y: f64,
    },
    Print(String),
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    SetFillColor(Color),
    SetLineWidth(f64),
    SetStrokeColor(Color),
    Stroke,
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

pub struct KotoWrapper {
    koto: Koto,
    play_module: ValueMap,
    user_data: Value,
    is_ready: bool,
    compiler_output: Element,
    script_output: Element,
    canvas: HtmlCanvasElement,
    canvas_context: CanvasRenderingContext2d,
    output_buffer: String,
    message_queue: KotoMessageQueue,
}

impl KotoWrapper {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let koto = Koto::with_settings(
            KotoSettings::default()
                .with_stdout(OutputCapture {})
                .with_stderr(OutputCapture {}),
        );

        let play_module = ValueMap::default();
        koto.prelude()
            .add_map("canvas", make_canvas_module(canvas.clone()));
        koto.prelude().add_map("play", play_module.clone());
        koto.prelude().add_map("random", koto_random::make_module());

        let canvas_context = canvas
            .get_context("2d")
            .expect("Error while getting canvas context")
            .expect("Missing canvas context")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Error while casting canvas context");

        canvas_context.set_fill_style(&JsValue::from("#999999"));
        canvas_context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        Self {
            koto,
            play_module,
            compiler_output: get_element_by_id("compiler-output"),
            script_output: get_element_by_id("script-output"),
            canvas,
            canvas_context,
            output_buffer: String::with_capacity(128),
            message_queue: KOTO_MESSAGE_QUEUE.with(|q| q.clone()),
            user_data: Value::Empty,
            is_ready: false,
        }
    }

    pub fn compile_script(&mut self, call_setup: bool) {
        let script = get_ace().edit("editor").get_session().get_value();

        self.is_ready = false;
        self.compiler_output.set_inner_html("");
        self.script_output.set_inner_html("");
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

        if let Err(e) = self.koto.run() {
            self.log_error(&e.to_string());
            return;
        }

        if call_setup {
            self.user_data = match self.play_module.data().get_with_string("setup") {
                Some(f) => match self.koto.run_function(f.clone(), CallArgs::None) {
                    Ok(data) => data,
                    Err(e) => {
                        self.log_error(&e.to_string());
                        return;
                    }
                },
                None => Value::Map(ValueMap::default()),
            };
        }

        if let Err(e) = self.run_play_function("on_load", &[self.user_data.clone()]) {
            self.log_error(&e.to_string());
            return;
        }

        self.is_ready = true;

        self.process_koto_messages();
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    pub fn update_should_be_called(&self) -> bool {
        self.is_ready && self.play_module.data().get_with_string("update").is_some()
    }

    fn log_error(&self, error: &str) {
        self.compiler_output
            .append_with_str_1(error)
            .expect("Failed to log error");
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

    pub fn run_update(&mut self, time_delta: f64) {
        debug_assert!(self.is_ready);

        match self.run_play_function("update", &[self.user_data.clone(), time_delta.into()]) {
            Ok(_) => {
                self.process_koto_messages();
            }
            Err(e) => {
                self.is_ready = false;
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
                        .expect("Failed to draw arc");
                }
                KotoMessage::BeginPath => {
                    self.canvas_context.begin_path();
                }
                KotoMessage::Clear => {
                    self.canvas_context.clear_rect(
                        0.0,
                        0.0,
                        self.canvas.width() as f64,
                        self.canvas.height() as f64,
                    );
                }
                KotoMessage::Fill => {
                    self.canvas_context.fill();
                }
                KotoMessage::MoveTo { x, y } => {
                    self.canvas_context.move_to(x, y);
                }
                KotoMessage::Print(s) => {
                    self.output_buffer.push_str(&s);
                }
                KotoMessage::Rect {
                    x,
                    y,
                    width,
                    height,
                } => self.canvas_context.rect(x, y, width, height),
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
                KotoMessage::Stroke => {
                    self.canvas_context.stroke();
                }
            }
        }

        if !self.output_buffer.is_empty() {
            self.script_output
                .append_with_str_1(&self.output_buffer)
                .expect("Failed to append to script output");
            self.script_output
                .set_scroll_top(self.script_output.scroll_height());
            self.output_buffer.clear();
        }
    }
}

fn send_koto_message(message: KotoMessage) {
    KOTO_MESSAGE_QUEUE.with(|q| q.borrow_mut().push_back(message));
}

fn make_canvas_module(canvas: HtmlCanvasElement) -> ValueMap {
    use Value::*;

    let result = ValueMap::default();

    result.add_fn("arc", |vm, args| {
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
        send_koto_message(KotoMessage::Arc {
            x,
            y,
            radius,
            start_angle,
            end_angle,
            counter_clockwise,
        });
        Ok(Empty)
    });

    result.add_fn("begin_path", |_, _| {
        send_koto_message(KotoMessage::BeginPath);
        Ok(Empty)
    });

    result.add_fn("clear", |_, _| {
        send_koto_message(KotoMessage::Clear);
        Ok(Empty)
    });

    result.add_fn("fill", |_, _| {
        send_koto_message(KotoMessage::Fill);
        Ok(Empty)
    });

    result.add_fn("height", {
        let canvas = canvas.clone();
        move |_, _| Ok(Number(canvas.width().into()))
    });

    result.add_fn("move_to", |vm, args| {
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
        send_koto_message(KotoMessage::MoveTo { x, y });
        Ok(Empty)
    });

    result.add_fn("rect", |vm, args| {
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
                    "x & y (as Numbers or a Num2), width, heigth",
                    unexpected,
                )
            }
        };
        send_koto_message(KotoMessage::Rect {
            x,
            y,
            width,
            height,
        });
        Ok(Empty)
    });

    result.add_fn("set_fill_color", |vm, args| {
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
        send_koto_message(KotoMessage::SetFillColor(Color { r, b, g, a }));
        Ok(Empty)
    });

    result.add_fn("set_line_width", |vm, args| {
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
        send_koto_message(KotoMessage::SetLineWidth(width.into()));
        Ok(Empty)
    });

    result.add_fn("set_stroke_color", |vm, args| {
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
        send_koto_message(KotoMessage::SetStrokeColor(Color { r, b, g, a }));
        Ok(Empty)
    });

    result.add_fn("stroke", |_, _| {
        send_koto_message(KotoMessage::Stroke);
        Ok(Empty)
    });

    result.add_fn("width", {
        let canvas = canvas.clone();
        move |_, _| Ok(Number(canvas.width().into()))
    });

    result
}

// Captures output from Koto in a String
#[derive(Debug)]
struct OutputCapture {}

impl KotoFile for OutputCapture {}
impl KotoRead for OutputCapture {}

impl KotoWrite for OutputCapture {
    fn write(&self, bytes: &[u8]) -> Result<(), RuntimeError> {
        let bytes_str = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string().into()),
        };
        KOTO_MESSAGE_QUEUE.with(|q| {
            q.borrow_mut()
                .push_back(KotoMessage::Print(bytes_str.to_string()))
        });
        Ok(())
    }

    fn write_line(&self, output: &str) -> Result<(), RuntimeError> {
        send_koto_message(KotoMessage::Print(format!("{output}\n")));
        Ok(())
    }

    fn flush(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

impl fmt::Display for OutputCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("_stdout_")
    }
}
