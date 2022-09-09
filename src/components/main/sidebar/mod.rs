use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{
        global::{friends::Friends, profile::Profile},
        main::sidebar::nav::{Nav, NavEvent},
        ui_kit::{
            button::Button, extension_placeholder::ExtensionPlaceholder, icon_button::IconButton,
            icon_input::IconInput,
        },
    },
    state::{Actions, PersistedState},
    Account, Messaging,
};

pub mod chat;
pub mod nav;

#[derive(Props, PartialEq)]
pub struct Props {
    state: PersistedState,
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<Props>) -> Element {
    let show_friends = use_state(&cx, || false);
    let show_profile = use_state(&cx, || false);

    let has_chats = !cx.props.state.chats.read().is_empty();

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
            },
            label {
                "Chats"
            },
            if has_chats {
                rsx!(
                    div {
                        cx.props.state.chats.read().clone().iter().rev().map(|conv| {
                            let conversation = conv.clone();
                            let state = cx.props.state.clone();
                            rsx!(
                                chat::Chat {
                                    state: cx.props.state.clone(),
                                    account: cx.props.account.clone(),
                                    conversation: conversation.clone(),
                                    on_pressed: move |_| {
                                        state.dispatch(Actions::ChatWith(conversation.clone())).save();
                                    }
                                }
                            )
                        })
                    }
                )
            } else {
                rsx!(
                    p {
                        "No active chats, yet.."
                    },
                    Button {
                        icon: Shape::Plus,
                        text: "Start One".to_string(),
                        on_pressed: move |_| show_friends.set(true),
                    },
                )
            },
            Friends {
                state: cx.props.state.clone(),
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
                title: "Friends".to_string(),
                show: *show_friends.clone(),
                icon: Shape::Users,
                on_hide: move |_| show_friends.set(false),
            },
            Profile {
                account: cx.props.account.clone(),
                show: *show_profile.clone(),
                on_hide: move |_| show_profile.set(false),
            },
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
                            show_profile.set(true);
                        },
                    }
                }
            }
        }
    })
}
