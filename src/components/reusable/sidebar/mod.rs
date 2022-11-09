use dioxus::prelude::*;

use crate::{
    components::{
        reusable::nav::{Nav, NavEvent},
        ui_kit::extension_placeholder::ExtensionPlaceholder,
    },
    utils::config::Config,
    Account,
};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    active: NavEvent,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Sidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let config = Config::load_config_or_default();

    cx.render(rsx! {
        div {
            id: "sidebar",
            config.developer.developer_mode.then(|| rsx! {
                ExtensionPlaceholder {},
            }),
            div {
                class: "children",
                &cx.props.children
            },
            Nav {
                account: cx.props.account.clone(),
                active: cx.props.active.clone(),
            }
        }
    })
}
