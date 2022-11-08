use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use uuid::Uuid;
use warp::raygun::Message;

use crate::{
    components::{
        main::{
            friends::Friends,
            profile::Profile,
            sidebar::{
                favorites::Favorites,
                nav::{Nav, NavEvent},
            },
        },
        ui_kit::{
            button::Button, extension_placeholder::ExtensionPlaceholder, icon_input::IconInput,
        },
    },
    extensions::*,
    state::{Actions, ConversationInfo},
    utils::{self, config::Config, notifications::PushNotification},
    Account, Messaging, LANGUAGE, STATE,
};

pub mod chat;
pub mod favorites;
pub mod nav;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<Props>) -> Element {
    let config = Config::load_config_or_default();
    let mp = cx.props.account.clone();

    let state = use_atom_ref(&cx, STATE);
    let show_profile = use_state(&cx, || false);

    let l = use_atom_ref(&cx, LANGUAGE).read();
    let friendString = l.friends.to_string();
    let noactivechatdString = l.no_active_chats.to_string();
    let chatsdString = l.chats.to_string();
    let has_chats = !state.read().all_chats.is_empty();

    let active_chat: UseState<Option<Uuid>> = use_state(&cx, || None).clone();
    let _active_chat = state.read().current_chat;
    if *active_chat != _active_chat {
        active_chat.set(_active_chat);
    }

    let exts = get_renders(ExtensionType::SidebarWidget, config.extensions.enable);

    let notifications_tx = use_coroutine(&cx, |mut rx: UnboundedReceiver<Message>| async move {
        while let Some(msg) = rx.next().await {
            let display_username = utils::get_username_from_did(msg.sender().clone(), &mp);
            PushNotification(display_username, msg.value().join("\n"));
        }
    });

    // sort the chats by time (ascending order)
    let mut chats: Vec<ConversationInfo> = state
        .read()
        .all_chats
        .iter()
        .map(|(_k, v)| v)
        .cloned()
        .collect();
    chats.sort();

    cx.render(rsx!{
        div {
            exts,
        }
        div {
            class: "sidebar",
            IconInput {
                icon: Shape::Search,
                placeholder: String::from("Search"),
                value: String::from(""),
                on_change: move |_| {},
                on_enter: move |_| {},
            },
            config.developer.developer_mode.then(|| rsx! {
                ExtensionPlaceholder {},
            }),
            Favorites {
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone()
            },
            label {
                style: "margin-bottom: 0;",
                "{chatsdString}"
            },
            if has_chats {
                rsx!(
                    div {
                        class: "chat_wrap",
                        div {
                            class: "chats",
                            // order the chats with most recent first (descending order)
                            chats.iter().rev().map(|conv| {
                                let key = conv.conversation.id();
                                let conversation_info = conv.clone();
                                let active_chat = active_chat.clone();
                                rsx!(
                                    chat::Chat {
                                        key: "{key}",
                                        account: cx.props.account.clone(),
                                        conversation_info: conversation_info.clone(),
                                        messaging: cx.props.messaging.clone(),
                                        last_msg_sent: conv.last_msg_sent.clone(),
                                        is_active: active_chat == Some(conversation_info.conversation.id()),
                                        tx_chan: notifications_tx.clone(),
                                        on_pressed: move |uuid| {
                                            if *active_chat != Some(uuid) {
                                                state.write().dispatch(Actions::ChatWith(conversation_info.clone()));
                                                active_chat.set(Some(uuid));
                                            }
                                        }
                                    }
                                )
                            })
                        }
                    }
                )
            } else {
                rsx!(
                    div {
                        class: "fill-vertical",
                        p {
                            "{noactivechatdString}"
                        },
                        div {
                            class: "m-bottom"
                        },
                        Button {
                            icon: Shape::Plus,
                            text: l.start_one.to_string(),
                            on_pressed: move |_| {

                            },
                        },
                    }
                )
            },
            Profile {
                account: cx.props.account.clone(),
                show: *show_profile.clone(),
                on_hide: move |_| show_profile.set(false),
            },
            Nav {
                account: cx.props.account.clone(),
                on_pressed: move | e: NavEvent | {
                    match e {
                        NavEvent::Home => {
                        },
                        NavEvent::Files => {
                            use_router(&cx).push_route("/main/files", None, None);
                        },
                        NavEvent::Friends => {
                            use_router(&cx).push_route("/main/friends", None, None);
                        },
                        NavEvent::Profile => {
                            show_profile.set(true);
                        },
                        NavEvent::Settings => {
                            use_router(&cx).push_route("/main/settings", None, None);
                        },
                    };
                }
            }
        }
    })
}
