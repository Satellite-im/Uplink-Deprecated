use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use utils::extensions::{get_renders, ExtensionType};
use uuid::Uuid;
use warp::raygun::Message;

use crate::{
    components::{main::sidebar::favorites::Favorites, reusable::nav::Nav},
    iutils::config::Config,
    state::{Actions, ConversationInfo},
    Messaging, LANGUAGE, STATE,
};

use ::utils::{notifications::PushNotification, Account};
use ui_kit::{
    context_menu::{ContextItem, ContextMenu},
    extension_placeholder::ExtensionPlaceholder,
    input::Input,
    skeletal_chats::SkeletalChats,
};

pub mod chat;
pub mod favorites;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<Props>) -> Element {
    log::debug!("rendering main/Sidebar");
    let config = Config::load_config_or_default();
    let mp = cx.props.account.clone();

    let state = use_atom_ref(&cx, STATE);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let chatsdString = l.chats.to_string();
    let has_chats = !state.read().active_chats.is_empty();

    let active_chat: UseState<Option<Uuid>> = use_state(&cx, || None).clone();
    let _active_chat = state.read().selected_chat;
    if *active_chat != _active_chat {
        active_chat.set(_active_chat);
    }

    let exts = get_renders(ExtensionType::SidebarWidget, config.extensions.enable);

    let notifications_tx = use_coroutine(&cx, |mut rx: UnboundedReceiver<Message>| async move {
        while let Some(msg) = rx.next().await {
            let display_username = crate::iutils::get_username_from_did(msg.sender().clone(), &mp);
            PushNotification(
                display_username,
                msg.value().join("\n"),
                ::utils::sounds::Sounds::Notification,
            );
        }
    });

    // sort the chats by time (ascending order)
    let mut chats: Vec<ConversationInfo> = state
        .read()
        .active_chats
        .iter()
        .map(|(_k, v)| v)
        .cloned()
        .collect();
    chats.sort();

    let fav_exist = !state.read().favorites.clone().is_empty();

    cx.render(rsx!{
        div {
            exts,
        }
        div {
            class: "sidebar",
            id: "main-sidebar",
            ContextMenu {
                parent: String::from("main-sidebar"),
                items: cx.render(rsx! {
                    ContextItem {
                        onpressed: move |_| use_router(&cx).push_route("/main/files", None, None),
                        text: String::from("Upload Files"),
                    },
                    ContextItem {
                        onpressed: move |_| use_router(&cx).push_route("/main/friends", None, None),
                        text: String::from("Manage Friends"),
                    },
                    ContextItem {
                        onpressed: move |_| use_router(&cx).push_route("/main/settings", None, None),
                        text: String::from("Settings"),
                    },
                })
            },
            Input {
                icon: Shape::MagnifyingGlass,
                placeholder: String::from("Search"),
                value: String::from(""),
                on_change: move |_| {},
                on_enter: move |_| {},
            },
            config.developer.developer_mode.then(|| rsx! {
                ExtensionPlaceholder {},
            }),
            fav_exist.then(|| rsx!{
                Favorites {
                    account: cx.props.account.clone(),
                    messaging: cx.props.messaging.clone()
                }
            }),
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
                                            // on press, change state so CSS class flips to show the chat
                                            state.write().dispatch(Actions::HideSidebar(true));
                                            if *active_chat != Some(uuid) {
                                                state.write().dispatch(Actions::ShowConversation(conversation_info.conversation.id()));
                                                active_chat.set(Some(uuid));
                                            }
                                        }
                                    }
                                )
                            })
                        }
                    }
                )
            } else { rsx!( SkeletalChats {}, div { class: "flex-1" } ) },
            Nav {
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
            }
        }
    })
}
