use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use self::nav::Route;
use crate::{components::main::settings::sidebar::nav::Nav, Account};
use ui_kit::icon_input::IconInput;

pub mod nav;

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, Route>,
    account: Account,
    initial_value: Route,
}

#[allow(non_snake_case)]
pub fn SettingsSidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering SettingsSidebar");
    let initial_value = match cx.props.initial_value {
        Route::Profile => Route::Profile,
        Route::Developer => Route::Developer,
        _ => Route::General,
    };

    cx.render(rsx! {
        crate::components::reusable::sidebar::Sidebar {
            account: cx.props.account.clone(),
            IconInput {
                icon: Shape::MagnifyingGlass,
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
