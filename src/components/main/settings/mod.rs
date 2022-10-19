use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{
        main::settings::{
            pages::{Developer, General},
            sidebar::Sidebar,
        },
        ui_kit::icon_button::{self, IconButton},
    },
    Account,
};

use self::sidebar::nav::NavEvent;

pub mod pages;
pub mod sidebar;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Settings<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let active_page = use_state(&cx, || NavEvent::Developer);

    cx.render(rsx! {
        div {
            id: "settings",
            div {
                class: "closer",
                onclick: move |_| cx.props.on_hide.call(()),
            }
            div {
                id: "content",
                Sidebar {
                    on_pressed: move |ne| {
                        active_page.set(ne);
                    }
                },
                div {
                    id: "page",
                    div {
                        class: "wrapper",
                        div {
                            class: "close_wrapper",
                            IconButton {
                                icon: Shape::X,
                                state: icon_button::State::Secondary,
                                on_pressed: move |_| {
                                    cx.props.on_hide.call(());
                                }
                            }
                        }
                        div {
                            class: "content",
                            match active_page.get() {
                                NavEvent::General => rsx!(General { account: cx.props.account.clone() }),
                                NavEvent::Developer => rsx!(Developer { account: cx.props.account.clone() }),
                                _ => rsx!(Developer { account: cx.props.account.clone() }),
                            }
                        }
                    }
                }
            }
        }
    })
}
