use dioxus::{events::FormEvent, prelude::*};
use dioxus_elements::input_data::keyboard_types::Code;
use dioxus_heroicons::{outline::Shape, Icon};

use crate::context_menu::{ContextItem, ContextMenu};

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
    cx.render(match &cx.props.value {
        Some(value) => rsx! {
            div {
                class: "icon-input",
                id: "TODO-icon_input-input",
                ContextMenu {
                    parent: String::from("TODO-icon_input-input"),
                    items: cx.render(rsx! {
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Paste"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Select All"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Copy"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Clear"),
                        },
                    })
                },
                Icon {
                    icon: cx.props.icon,
                },
                input {
                    class: "input",
                    placeholder: "{cx.props.placeholder}",
                    oninput: |e| cx.props.on_change.call(e),
                    value:"{value}",
                    onkeyup: |evt| {
                        if evt.code().eq(&Code::Enter) {
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
                        if evt.code().eq(&Code::Enter) {
                            cx.props.on_enter.call(())
                        }
                    }
                },
            }

        },
    })
}
