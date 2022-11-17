use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_html::KeyCode;

#[derive(PartialEq, Eq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    icon: Shape,
    #[props(optional)]
    value: Option<String>,
    on_change: EventHandler<'a, FormEvent>,
    on_enter: EventHandler<'a, ()>,
    placeholder: String,
}

// todo: stop re-rendering this element (and the parent element) on every keystroke
#[allow(non_snake_case)]
pub fn IconInput<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    log::debug!("rendering IconInput");
    cx.render(match &cx.props.value {
        Some(value) => rsx! {
            div {
                class: "icon-input",
                Icon {
                    icon: cx.props.icon,
                },
                input {
                    class: "input",
                    placeholder: "{cx.props.placeholder}",
                    oninput: |e| cx.props.on_change.call(e),
                    value:"{value}",
                    onkeyup: |evt| {
                        if evt.key_code == KeyCode::Enter {
                            cx.props.on_enter.call(())
                        }
                    }
                },
            }
        },
        None => rsx! {
            div {
                class: "icon-input",
                Icon {
                    icon: cx.props.icon,
                },
                input {
                    class: "input",
                    r#type:"search",
                    placeholder: "{cx.props.placeholder}",
                    oninput: |e| cx.props.on_change.call(e),
                    onkeyup: |evt| {
                        if evt.key_code == KeyCode::Enter {
                            cx.props.on_enter.call(())
                        }
                    }
                },
            }

        },
    })
}
