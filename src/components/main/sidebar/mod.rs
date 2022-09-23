use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{
        main::{friends::Friends, profile::Profile},
        main::sidebar::nav::{Nav, NavEvent},
        ui_kit::{
            button::Button, extension_placeholder::ExtensionPlaceholder, icon_button::IconButton,
            icon_input::IconInput,
        },
    },
    state::Actions,
    Account, Messaging, STATE,
};

pub mod chat;
pub mod nav;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<Props>) -> Element {
    let show_friends = use_state(&cx, || false);
    let show_profile = use_state(&cx, || false);
    let state = use_atom_ref(&cx, STATE);

    let has_chats = !state.read().chats.clone().is_empty();

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
                        state.read().chats.iter().rev().map(|conv| {
                            let conversation = conv.clone();
                            rsx!(
                                chat::Chat {
                                    account: cx.props.account.clone(),
                                    conversation: conversation.clone(),
                                    on_pressed: move |_| {
                                        state.write().dispatch(Actions::ChatWith(conversation.clone())).save();
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
