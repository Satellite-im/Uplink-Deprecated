use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use ui_kit::switch::Switch;
use utils::extensions::ExtensionInfo;

#[derive(Props, PartialEq)]
pub struct Props {
    extension: ExtensionInfo,
}

#[allow(non_snake_case)]
pub fn ExtensionOptions(cx: Scope<Props>) -> Element {
    log::debug!("rendering extension settings");

    cx.render(rsx! {
        div {
            class: "extension",
            div {
                class: "header",
                div {
                    class: "icon",
                    Icon {
                        icon: Shape::Beaker
                    }
                },
                div {
                    class: "details",
                    h2 {
                        "{cx.props.extension.name}",
                    }
                    p {
                        "{cx.props.extension.author}",
                    }
                }
                div {
                    class: "toggle",
                    Switch {
                        active: true,
                        on_change: move |_| {}
                    }
                }
            }
            p {
                class: "desc",
                "{cx.props.extension.description}"
            }
        }
    })
}
