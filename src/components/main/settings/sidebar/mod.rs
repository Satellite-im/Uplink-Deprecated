use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{main::settings::sidebar::nav::Nav, ui_kit::icon_input::IconInput},
    Account,
};

use self::nav::NavEvent;

pub mod nav;

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, NavEvent>,
    account: Account,
    initial_value: NavEvent,
}

#[allow(non_snake_case)]
pub fn SettingsSidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering SettingsSidebar");
    let initial_value = match cx.props.initial_value {
        NavEvent::Profile => NavEvent::Profile,
        NavEvent::Developer => NavEvent::Developer,
        _ => NavEvent::General,
    };

    cx.render(rsx! {
        crate::components::reusable::sidebar::Sidebar {
            account: cx.props.account.clone(),
            IconInput {
                icon: Shape::Search,
                placeholder: String::from("Search"),
                value: String::from(""),
                on_change: move |_| {},
                on_enter: move |_| {},
            },
            Nav {
                on_pressed: move |ne| {
                    cx.props.on_pressed.call(ne);
                }
                initial_value: initial_value,
            },
        },
    })
}
