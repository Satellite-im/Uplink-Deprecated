use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{components::{ui_kit::{icon_input::IconInput, extension_placeholder::ExtensionPlaceholder}, main::settings::sidebar::nav::Nav}, utils::config::Config};

use self::nav::NavEvent;

pub mod nav;

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Sidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let config = Config::load_config_or_default();

    cx.render(rsx! {
        div {
            id: "sidebar",
            IconInput {
                icon: Shape::Search,
                placeholder: String::from("Search"),
                value: String::from(""),
                on_change: move |_| {},
                on_enter: move |_| {},
            },
            config.developer.developer_mode.then(|| rsx! {
                ExtensionPlaceholder {},
            })
            Nav {
                on_pressed: move |ne| {
                    let _ = cx.props.on_pressed.call(ne);
                }
            },
            config.developer.developer_mode.then(|| rsx! {
                ExtensionPlaceholder {},
            })
        },
    })
}
