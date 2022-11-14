use dioxus::prelude::*;

use crate::{
    components::main::settings::pages::{developer::Developer, general::General, profile::Profile},
    Account,
};

use self::sidebar::nav::NavEvent;

pub mod pages;
pub mod sidebar;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    page_to_open: NavEvent,
}

#[allow(non_snake_case)]
pub fn Settings(cx: Scope<Props>) -> Element {
    let page_to_open_on_settings = match cx.props.page_to_open {
        NavEvent::Profile => NavEvent::Profile,
        NavEvent::Developer => NavEvent::Developer,
        _ => NavEvent::General,
    };

    let active_page = use_state(&cx, || page_to_open_on_settings);

    cx.render(rsx! {
        div {
            id: "settings",
            sidebar::SettingsSidebar {
                account: cx.props.account.clone(),
                on_pressed: move |ne| {
                    active_page.set(ne);
                },
                initial_value: match active_page.get() {
                    NavEvent::Profile => NavEvent::Profile,
                    NavEvent::Developer => NavEvent::Developer,
                    _ => NavEvent::General,
                },
            },
            div {
                id: "content",
                div {
                    id: "page",
                    div {
                        class: "wrapper",
                        div {
                            class: "content",
                            match active_page.get() {
                                NavEvent::General => rsx!(General { account: cx.props.account.clone() }),
                                NavEvent::Developer => rsx!(Developer { account: cx.props.account.clone() }),
                                NavEvent::Profile => rsx!(Profile { account: cx.props.account.clone() }),
                                _ => rsx!(Developer { account: cx.props.account.clone() }),
                            }
                        }
                    }
                }
            }
        }
    })
}
