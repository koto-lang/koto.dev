use {
    crate::{
        ace_bindings::{get_ace, AceEditor, AceSession},
        get_element_by_id, get_local_storage_value,
        koto_wrapper::KotoWrapper,
        set_local_storage_value, SCRIPTS,
    },
    gloo_events::EventListener,
    gloo_render::AnimationFrame,
    gloo_utils::window,
    std::{cell::RefCell, rc::Rc},
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::{HtmlCanvasElement, HtmlOptGroupElement, HtmlOptionElement, HtmlSelectElement},
};

pub struct App {
    koto: KotoWrapper,
    editor: AceEditor,
    canvas: HtmlCanvasElement,
    last_time: Option<f64>,
    current_time: f64,
    animation_frame: Option<AnimationFrame>,
    event_listeners: Vec<EventListener>,
    self_rc: Option<Rc<RefCell<App>>>,
    vim_bindings_enabled: bool,
}

impl App {
    pub fn setup() {
        let canvas = get_element_by_id("koto-canvas");
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let editor = setup_editor();
        let editor_session = editor.get_session();
        let (menu, scripts) = setup_script_menu();

        let mut app = Self {
            koto: KotoWrapper::new(canvas.clone()),
            editor,
            canvas,
            last_time: None,
            current_time: 0.0,
            animation_frame: None,
            event_listeners: Vec::new(),
            self_rc: None,
            vim_bindings_enabled: false,
        };

        app.set_vim_bindings_enabled(
            get_local_storage_value("vim-bindings-enabled")
                .map_or(false, |enabled| enabled == "true"),
        );

        let app_rc = Rc::new(RefCell::new(app));

        let app_clone = app_rc.clone();
        app_rc.borrow_mut().self_rc = Some(app_clone.clone());
        app_rc
            .borrow_mut()
            .setup_listeners(editor_session, menu, scripts);
        app_rc.borrow_mut().on_script_edited();
    }

    pub fn on_script_edited(&mut self) {
        let script = self.editor.get_session().get_value();
        if !script.is_empty() {
            self.koto.compile_script(&script);

            if self.koto.update_should_be_called() {
                self.request_animation_frame();
            }
        }
    }

    fn request_animation_frame(&mut self) {
        self.animation_frame = Some(gloo_render::request_animation_frame({
            let app = self.self_rc.as_ref().unwrap().clone();
            move |time| app.borrow_mut().on_animation_frame(time)
        }));
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
        self.koto.on_resize();
    }

    pub fn reset(&mut self) {
        self.koto.reset();
        self.animation_frame = None;
        self.last_time = None;
    }

    pub fn editor_session(&self) -> AceSession {
        self.editor.get_session()
    }

    fn on_vim_bindings_clicked(&mut self) {
        self.set_vim_bindings_enabled(!self.vim_bindings_enabled);
    }

    fn set_vim_bindings_enabled(&mut self, enabled: bool) {
        self.vim_bindings_enabled = enabled;
        if self.vim_bindings_enabled {
            get_element_by_id("vim-bindings").set_class_name("uk-button-primary");
            self.editor.set_keyboard_handler("ace/keyboard/vim");
        } else {
            get_element_by_id("vim-bindings").set_class_name("uk-button-default");
            self.editor.set_keyboard_handler("");
        }
    }

    fn setup_listeners(
        &mut self,
        editor_session: AceSession,
        script_menu: HtmlSelectElement,
        scripts: Vec<&'static str>,
    ) {
        let app = self.self_rc.as_ref().unwrap().clone();

        {
            let on_change = Closure::wrap({
                let app = app.clone();
                Box::new(move || app.borrow_mut().on_script_edited())
            } as Box<dyn FnMut()>);
            editor_session.on("change", on_change.as_ref().unchecked_ref());
            on_change.forget();
        }

        self.event_listeners = vec![
            EventListener::new(&window(), "beforeunload", {
                let app = app.clone();
                move |_| {
                    let app = app.borrow();
                    set_local_storage_value("script", &app.editor.get_session().get_value());
                    set_local_storage_value(
                        "vim-bindings-enabled",
                        if app.vim_bindings_enabled {
                            "true"
                        } else {
                            "false"
                        },
                    );
                }
            }),
            EventListener::new(&window(), "resize", {
                let app = app.clone();
                move |_| app.borrow_mut().on_window_resize()
            }),
            EventListener::new(&script_menu.clone(), "change", {
                let app = app.clone();
                move |_| {
                    let script_index = script_menu.selected_index();
                    if script_index > 0 {
                        let script = scripts[script_index as usize - 1];
                        app.borrow_mut().reset();
                        let editor_session = app.borrow().editor_session();
                        editor_session.set_value(script);
                    }
                }
            }),
            EventListener::new(&get_element_by_id("vim-bindings"), "click", {
                let app = app.clone();
                move |_| app.borrow_mut().on_vim_bindings_clicked()
            }),
        ];
    }
}

fn setup_editor() -> AceEditor {
    let ace = get_ace();
    let editor = ace.edit("editor");
    editor.set_theme("ace/theme/solarized_dark");
    editor.set_show_print_margin(false);

    let session = editor.get_session();
    session.set_mode("ace/mode/koto");
    session.set_use_soft_tabs(true);
    session.set_tab_size(2);

    session.set_value(
        get_local_storage_value("script")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(include_str!("scripts/canvas/random_rects.koto")),
    );

    editor
}

fn setup_script_menu() -> (HtmlSelectElement, Vec<&'static str>) {
    let document = gloo_utils::document();
    let menu = get_element_by_id("select-script")
        .dyn_into::<HtmlSelectElement>()
        .unwrap();

    let mut scripts = Vec::new();

    for group in SCRIPTS {
        let group_element = document
            .create_element("optgroup")
            .unwrap()
            .dyn_into::<HtmlOptGroupElement>()
            .unwrap();
        group_element.set_label(group.name);

        for script in group.scripts {
            let option = document
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            option.set_text(script.name);
            option.set_default_selected(false);
            group_element.append_child(&option).unwrap();
            scripts.push(script.script);
        }

        menu.append_child(&group_element)
            .expect("Failed to append script group");
    }

    (menu, scripts)
}
