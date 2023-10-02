use {
    koto::prelude::*,
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
    web_sys::Element,
};

pub type KotoMessageQueue = Rc<RefCell<VecDeque<KotoMessage>>>;

pub enum KotoMessage {
    Print(String),
}

pub struct KotoWrapper {
    koto: Koto,
    compiler_output: Element,
    script_output: Element,
    output_buffer: String,
    message_queue: KotoMessageQueue,
}

impl KotoWrapper {
    pub fn new(compiler_output: Element, script_output: Element) -> Self {
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

        Self {
            koto,
            compiler_output,
            script_output,
            output_buffer: String::with_capacity(128),
            message_queue,
        }
    }

    pub fn compile_and_run_script(&mut self, script: &str) {
        debug_assert!(!script.is_empty());

        self.compiler_output.set_inner_html("");
        self.script_output.set_inner_html("");
        self.message_queue.borrow_mut().clear();

        self.koto.exports().data_mut().clear();
        self.koto.clear_module_cache();

        if let Err(error) = self.koto.compile(&script) {
            self.error(&format!("Error while compiling script: {error}"));
        } else {
            self.compiler_output.set_inner_html("Success");

            if let Err(e) = self.koto.run() {
                return self.error(&e.to_string());
            }

            self.script_output.set_inner_html("");
            self.process_koto_messages();
        }
    }

    fn error(&mut self, error: &str) {
        self.compiler_output.set_inner_html(error);
        self.compiler_output
            .set_scroll_top(self.compiler_output.scroll_height());
    }

    fn process_koto_messages(&mut self) {
        for message in self.message_queue.borrow_mut().drain(..) {
            match message {
                KotoMessage::Print(s) => self.output_buffer.push_str(&s),
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
