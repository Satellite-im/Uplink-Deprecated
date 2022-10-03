use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{components::{ui_kit::{icon_input::IconInput, extension_placeholder::ExtensionPlaceholder}, main::settings::sidebar::nav::Nav}};

use self::nav::NavEvent;

pub mod nav;

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Sidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            id: "sidebar",
            IconInput {
                icon: Shape::Search,
                placeholder: "Search".to_string(),
                value: "".to_string(),
                on_change: move |_| {},
                on_enter: move |_| {},
            },
            ExtensionPlaceholder {},
            Nav {
                on_pressed: move |ne| {
                    let _ = cx.props.on_pressed.call(ne);
                }
            },
            ExtensionPlaceholder {},
        },
    })
}
