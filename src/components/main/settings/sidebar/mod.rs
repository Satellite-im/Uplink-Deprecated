use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{
        main::settings::sidebar::nav::Nav,
        ui_kit::{extension_placeholder::ExtensionPlaceholder, icon_input::IconInput},
    },
    utils::config::Config,
};

use self::nav::NavEvent;

pub mod nav;

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, NavEvent>,
    initial_value: NavEvent,
}

#[allow(non_snake_case)]
pub fn SettingsSidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let initial_value = match cx.props.initial_value {
        NavEvent::Profile => NavEvent::Profile,
        NavEvent::Developer => NavEvent::Developer,
        _ => NavEvent::General,
    };
    let config = Config::load_config_or_default();

    cx.render(rsx! {
        div {
            class: "app-sidebar",
            div {
                class: "sidebar-content",
                div {
                    class: "sidebar-section",
                    IconInput {
                        icon: Shape::Search,
                        placeholder: String::from("Search"),
                        value: String::from(""),
                        on_change: move |_| {},
                        on_enter: move |_| {},
                    },
                }
                div {
                class: "sidebar-scroll",
                    config.developer.developer_mode.then(|| rsx! {
                        ExtensionPlaceholder {},
                    })
                    Nav {
                        on_pressed: move |ne| {
                            cx.props.on_pressed.call(ne);
                        }
                        initial_value: initial_value,
                    },
                    config.developer.developer_mode.then(|| rsx! {
                        ExtensionPlaceholder {},
                    })
                }
            }
        },
    })
}
