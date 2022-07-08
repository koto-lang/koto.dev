use {
    super::playground::PlaygroundContext,
    crate::{copy_text_to_clipboard, show_notification},
    cloned::cloned,
    gloo_events::EventListener,
    gloo_net::http::Request,
    gloo_utils::window,
    js_sys::encode_uri_component,
    serde::{Deserialize, Serialize},
    wasm_bindgen::prelude::*,
    web_sys::Element,
    yew::{context::ContextHandle, prelude::*},
};

pub enum Msg {
    PlaygroundContextChanged(PlaygroundContext),
    GistCreated(CreateGistResponse),
    GistRequestError { error: String },
    GistResponseError { error: String },
    CopyPlaygroundUrl,
    CopyGistUrl,
    CopyTextUrl,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub script: String,
    pub on_hidden: Callback<()>,
}

enum GistState {
    CreatingGist,
    Success {
        playground_url: String,
        gist_url: String,
    },
    Error(String),
}

pub struct Share {
    gist_state: GistState,
    text_url: String,
    self_ref: NodeRef,
    on_hidden_listener: Option<EventListener>,
    playground_context: PlaygroundContext,
    _context_listener: ContextHandle<PlaygroundContext>,
}

impl Component for Share {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let location = window().location();
        let origin = location.origin().expect("Missing location origin");
        let path = location.pathname().expect("Missing location pathname");

        let text_url = format!(
            "{origin}{path}?script={}",
            encode_uri_component(&ctx.props().script)
        );

        ctx.link().send_future({
            let script = ctx.props().script.clone();
            async move {
                match Request::post("/create-gist").body(&script).send().await {
                    Ok(response) => match response.json::<CreateGistResponse>().await {
                        Ok(gist) => Msg::GistCreated(gist),
                        Err(error) => Msg::GistResponseError {
                            error: error.to_string(),
                        },
                    },
                    Err(error) => Msg::GistRequestError {
                        error: error.to_string(),
                    },
                }
            }
        });

        let (playground_context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::PlaygroundContextChanged))
            .expect("Missing playground context");

        Self {
            gist_state: GistState::CreatingGist,
            text_url,
            self_ref: NodeRef::default(),
            on_hidden_listener: None,
            playground_context,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlaygroundContextChanged(context) => {
                self.playground_context = context;
                true
            }
            Msg::GistCreated(gist) => {
                let location = window().location();

                let playground_url = format!(
                    "{origin}{path}?gist={id}",
                    origin = location.origin().expect("Missing location origin"),
                    path = location.pathname().expect("Missing location pathname"),
                    id = gist.id
                );

                self.gist_state = GistState::Success {
                    playground_url,
                    gist_url: gist.url,
                };
                true
            }
            Msg::GistRequestError { .. } => {
                self.gist_state = GistState::Error(format!("Error while requesting gist"));
                true
            }
            Msg::GistResponseError { error } => {
                self.gist_state = GistState::Error(format!("Error in gist response: '{error}'"));
                true
            }
            Msg::CopyPlaygroundUrl => match &self.gist_state {
                GistState::Success { playground_url, .. } => {
                    copy_link_to_clipboard(playground_url);
                    false
                }
                _ => false,
            },
            Msg::CopyGistUrl => match &self.gist_state {
                GistState::Success { gist_url, .. } => {
                    copy_link_to_clipboard(gist_url);
                    false
                }
                _ => false,
            },
            Msg::CopyTextUrl => {
                copy_link_to_clipboard(&self.text_url);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let body = match &self.gist_state {
            GistState::CreatingGist => {
                html! {
                    <p class="uk-text-center">
                        <div uk-spinner=""></div>
                    </p>
                }
            }
            _ => {
                let gist_links = match &self.gist_state {
                    GistState::Success {
                        playground_url,
                        gist_url,
                    } => {
                        html! {
                            <>
                                <ShareLink
                                    caption={"Playground link"}
                                    url={playground_url.clone()}
                                    on_copy_clicked={
                                        ctx.link().callback(|_| Msg::CopyPlaygroundUrl)
                                    }
                                />

                                <ShareLink
                                    caption={"Gist link"}
                                    url={gist_url.clone()}
                                    on_copy_clicked={ctx.link().callback(|_| Msg::CopyGistUrl)}
                                />
                            </>
                        }
                    }
                    GistState::Error(error) => {
                        html! {
                            <div uk-alert="" class="uk-alert-danger">
                                <span uk-icon="warning" class="uk-margin-small-right"></span>

                                {error}
                            </div>
                        }
                    }
                    GistState::CreatingGist => unreachable!(),
                };

                html! {
                    <div class="uk-card uk-card-body uk-border-rounded">
                        <ul class="uk-list">
                            {gist_links}

                            <ShareLink
                                caption={"Playground link with code in URL"}
                                url={self.text_url.clone()}
                                on_copy_clicked={ctx.link().callback(|_| Msg::CopyTextUrl)}
                            />
                        </ul>
                    </div>
                }
            }
        };

        let mut dialog_classes = classes![
            "uk-modal-dialog",
            "uk-modal-body",
            "uk-margin-auto-vertical",
            "uk-border-rounded"
        ];

        // UIkit doesn't currently have built-in support for inverse colours in modal dialogs
        if self.playground_context.dark_mode {
            dialog_classes.push("uk-light");
            dialog_classes.push("uk-background-secondary");
        }

        html! {
            <div uk-modal="" class="uk-flex-top" ref={self.self_ref.clone()}>
                <div class={dialog_classes}>
                    <h4 class="uk-modal-title uk-text-lighter">
                        {"Share"}
                    </h4>

                    {body}

                    <p class="uk-text-right">
                        <button class="uk-button uk-button-link uk-modal-close uk-text-capitalize" type="button">
                            {"Close"}
                        </button>
                    </p>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let modal_element = self.self_ref.cast::<Element>().unwrap();

            show_modal(modal_element.clone());

            self.on_hidden_listener = Some(EventListener::new(&modal_element, "hidden", {
                let on_hidden = ctx.props().on_hidden.clone();
                move |_| on_hidden.emit(())
            }));
        }
    }
}

fn copy_link_to_clipboard(link: &str) {
    copy_text_to_clipboard(&link);
    show_notification("Link copied to clipboard", "link");
}

#[wasm_bindgen(inline_js = "export function show_modal(element) { UIkit.modal(element).show() }")]
extern "C" {
    pub fn show_modal(element: Element);
}

#[derive(Serialize)]
struct CreateGistRequest {
    script: String,
}

#[derive(Deserialize)]
pub struct CreateGistResponse {
    id: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct ShareLinkProps {
    caption: String,
    url: String,
    on_copy_clicked: Callback<()>,
}

#[function_component(ShareLink)]
fn share_link(props: &ShareLinkProps) -> Html {
    let link_classes = classes!("uk-button", "uk-button-link", "uk-text-capitalize");

    html! {
        <li>
            <a class={link_classes.clone()}
                onclick={
                    Callback::from({
                        cloned!(props.on_copy_clicked);
                        move |_| on_copy_clicked.emit(())
                    })
                }
            >
                <span uk-icon="copy" class="uk-icon-link uk-margin-small-right"></span>
            </a>
            <a class={link_classes} href={props.url.clone()}>
                {props.caption.clone()}
            </a>
        </li>
    }
}
