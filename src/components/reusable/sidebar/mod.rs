use dioxus::prelude::*;
use ui_kit::extension_placeholder::ExtensionPlaceholder;

use crate::{components::reusable::nav::Nav, utils_internal::config::Config};
use ::utils::Account;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Sidebar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering reusable Sidebar");
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
            }
        }
    })
}
