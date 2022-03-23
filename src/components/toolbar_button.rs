use {cloned::cloned, yew::prelude::*};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon: &'static str,
    #[prop_or(1.0)]
    pub size: f32,
    #[prop_or("uk-button-default")]
    pub toggle_off_class: &'static str,
    #[prop_or("uk-button-primary")]
    pub toggle_on_class: &'static str,
    #[prop_or(false)]
    pub toggled: bool,
    pub on_clicked: Callback<()>,
}

#[function_component(ToolbarButton)]
pub fn toolbar_button(props: &Props) -> Html {
    html! {
        <button
            class={
                if props.toggled {
                    props.toggle_on_class
                } else {
                    props.toggle_off_class
                }
            }
            onclick={
                Callback::from({
                    cloned!(props.on_clicked);
                    move |_| on_clicked.emit(())
                })
            }
        >
            <span uk-icon={format!("icon: {}; ratio: {}", props.icon, props.size)}></span>
        </button>
    }
}
