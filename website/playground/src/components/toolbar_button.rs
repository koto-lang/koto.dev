use {cloned::cloned, yew::prelude::*};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon_left: Option<&'static str>,
    pub icon_right: Option<&'static str>,
    pub caption: Option<&'static str>,
    pub tooltip: Option<&'static str>,
    #[prop_or("uk-button-default")]
    pub toggle_off_class: &'static str,
    #[prop_or("uk-button-primary")]
    pub toggle_on_class: &'static str,
    #[prop_or(false)]
    pub toggled: bool,
    pub on_clicked: Option<Callback<()>>,
    pub uk_toggle: Option<&'static str>,
}

#[function_component(ToolbarButton)]
pub fn toolbar_button(props: &Props) -> Html {
    let icon_left = if let Some(icon) = props.icon_left {
        html! {
            <span uk-icon={icon}></span>
        }
    } else {
        html! {}
    };

    let caption = {
        let mut class = classes!("uk-text-light");
        if props.icon_left.is_some() {
            class.push("uk-margin-small-left");
        };

        if let Some(caption) = props.caption {
            html! { <span {class}>{caption}</span> }
        } else {
            html! {}
        }
    };

    let icon_right = {
        let class = if props.icon_left.is_some() || props.caption.is_some() {
            classes!("uk-margin-small-left")
        } else {
            classes!()
        };

        if let Some(icon) = props.icon_right {
            html! { <span {class} uk-icon={icon}></span> }
        } else {
            html! {}
        }
    };

    let uk_toggle = props.uk_toggle.map(|target| format!("target: #{target}"));

    html! {
        <button
            uk-tooltip={
                if let Some(tooltip) = props.tooltip {
                    format!("title: {}; delay: 1000", tooltip)
                } else {
                    String::default()
                }
            }
            class={classes!(
                "uk-button",
                "uk-button-small",
                "uk-text-capitalize",
                if props.toggled {
                    props.toggle_on_class
                } else {
                    props.toggle_off_class
                }
            )}
            onclick={
                if let Some(on_clicked) = &props.on_clicked {
                    Callback::from({
                        cloned!(on_clicked);
                        move |_| on_clicked.emit(())
                    })
                } else {
                    Callback::default()
                }
            }
            uk-toggle={uk_toggle}
        >
            {icon_left}
            {caption}
            {icon_right}
        </button>
    }
}
