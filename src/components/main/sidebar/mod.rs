use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;
use warp::tesseract::Tesseract;

use crate::components::{global::friends::Friends, ui_kit::{icon_button::IconButton, button::Button, extension_placeholder::ExtensionPlaceholder, icon_input::IconInput}, main::sidebar::nav::{Nav, NavEvent}};

pub mod nav;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope) -> Element {
    let show_friends = use_state(&cx, || false);

    global_css! {"
        .main {
            .sidebar {
                height: calc(100% - 2rem);
                min-width: 300px;
                position: relative;
                padding: 1rem;
                display: inline-flex;
                flex-direction: column;
                border-right: 1px solid var(--theme-borders);
                
                .extension-renderer {
                    margin-top: 1rem;
                }
                .favorites {
                    display: inline-flex;

                    .labeled {
                        position: relative;
                        padding: 0.75rem;
                        margin-bottom: 10px;
                        span {
                            position: absolute;
                            font-size: 9pt;
                            color: var(--theme-text-muted);
                            bottom: -10px;
                            left: 0;
                            right: 0;
                            text-align: center;
                        }
                    }
                }
            }
        }
    "}
    cx.render(rsx!{
        div {
            class: "sidebar",
            IconInput {
                icon: Shape::Search,
                placeholder: "Search".to_string(),
                value: "".to_string(),
                on_change: move |_| {},
                on_enter: move |_| {},
            },
            ExtensionPlaceholder {},
            label {
                "Favorites"
            },
            div {
                class: "favorites",
                div {
                    class: "labeled",
                    IconButton {
                        icon: Shape::Plus,
                        on_pressed: move |_| {},
                    },
                    span {
                        "New Chat"
                    }
                },
            }
            label {
                "Chats"
            },
            p {
                "No active chats, yet.."
            },
            Button {
                icon: Shape::UserAdd,
                text: "Add Someone".to_string(),
                on_pressed: move |_| show_friends.set(true),
            },
            Friends {
                title: "Friends".to_string(),
                show: *show_friends.clone(),
                icon: Shape::Users,
                onclick: move |_| show_friends.set(false),
            }
            Nav {
                on_pressed: move | e: NavEvent | {
                    show_friends.set(false);

                    match e {
                        NavEvent::Home => {
                        },
                        NavEvent::Files => {
                        },
                        NavEvent::Friends => {
                            show_friends.set(true);
                        },
                        NavEvent::Profile => {
                        },
                    }
                }
            }
        }
    })
}