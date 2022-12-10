use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use ui_kit::switch::Switch;
use utils::extensions::ExtensionInfo;
use crate::STATE;
use crate::state::Actions;

#[derive(Props, Eq, PartialEq)]
pub struct Props {
    extension: ExtensionInfo,
}

#[allow(non_snake_case)]
pub fn ExtensionOptions(cx: Scope<Props>) -> Element {
    log::debug!("rendering extension settings");

    let state = use_atom_ref(&cx, STATE);
    let name = &cx.props.extension.name;
    let is_enabled = state.read().enabled_extensions.contains(name);
    let toggle = move |_| state.write().dispatch(
        Actions::SetExtensionEnabled(name.clone(), !is_enabled)
    );


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
                        active: is_enabled,
                        on_change: toggle
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
