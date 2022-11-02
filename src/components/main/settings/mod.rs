use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{
        main::settings::pages::{developer::Developer, general::General, profile::Profile},
        ui_kit::icon_button::{self, IconButton},
    },
    Account,
};

use self::sidebar::nav::NavEvent;

pub mod pages;
pub mod sidebar;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Settings(cx: Scope<Props>) -> Element {
    let active_page = use_state(&cx, || NavEvent::Developer);

    cx.render(rsx! {
        div {
            id: "settings",
            sidebar::SettingsSidebar {
                on_pressed: move |ne| {
                    active_page.set(ne);
                },
            },
            div {
                id: "content",
                div {
                    style: "align-self: flex-end; padding: 1rem; padding-bottom: 0;",
                    IconButton {
                        on_pressed: move |_| use_router(&cx).push_route("/main", None, None),
                        icon: Shape::X,
                        state: icon_button::State::Secondary,
                    },
                },
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
