use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use dioxus::{events::FormEvent, prelude::*, router::RouterCore};
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
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
    let router = use_router(&cx).clone();
    let router2 = router.clone();
    let router3 = router.clone();

    let state = use_atom_ref(&cx, STATE);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let chatsdString = l.chats.to_string();
    let has_chats = !state.read().active_chats.is_empty();

    let search_value = use_state(&cx, String::new);
    let participant_usernames: RefCell<HashMap<Uuid, String>> = RefCell::new(HashMap::new());

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

    let mp = cx.props.account.clone();
    let ident = mp.get_own_identity().expect("Unexpected error <temp>");

    let matcher = SkimMatcherV2::default();
    let filtered_chats = chats.clone().into_iter().filter(|conv| {
        if search_value.get().is_empty() {
            return true;
        }

        if participant_usernames
            .borrow()
            .get(&conv.conversation.id())
            .is_none()
        {
            let username = conv
                .clone()
                .conversation
                .recipients()
                .iter()
                .filter(|did| ident.did_key().ne(did))
                .filter_map(|did| mp.get_identity(did.clone().into()).ok())
                .flatten()
                .map(|i| i.username())
                .last()
                .unwrap_or_default();

            participant_usernames
                .borrow_mut()
                .insert(conv.conversation.id(), username);
        }

        let search = search_value.clone().get().to_lowercase();
        let mut score = 0;
        let conv_id = conv.borrow().clone().conversation.id();

        if matcher
            .fuzzy_match(&participant_usernames.borrow()[&conv_id], &search)
            .is_some()
        {
            score += 1;
        }
        score >= 1
    });

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
                        onpressed:  move |_|router.replace_route("/main/files", None, None),
                        text: String::from("Upload Files"),
                    },
                    ContextItem {
                        onpressed:  move |_|router2.replace_route("/main/friends", None, None),
                        text: String::from("Manage Friends"),
                    },
                    ContextItem {
                        onpressed: move |_| router3.replace_route("/main/settings", None, None),
                        text: String::from("Settings"),
                    },
                })
            },
            div {
                class: "search-input",
                Input {
                    icon: Shape::MagnifyingGlass,
                    placeholder: String::from("Search"),
                    value: search_value.to_string(),
                    on_change: move |e: FormEvent| {
                        search_value.set(e.value.clone());
                    },
                    on_enter: move |_| {},
                },
            }
            config.developer.developer_mode.then(|| rsx! {
                ExtensionPlaceholder {},
            }),
            fav_exist.then(|| rsx!{
                Favorites {
                    account: cx.props.account.clone(),
                    messaging: cx.props.messaging.clone()
                }
            }),
            div {
                class: "label chat-label",
                "{chatsdString}"
            },
            if has_chats {
                rsx!(
                    div {
                        class: "chats",
                        // order the chats with most recent first (descending order)
                        filtered_chats.rev().map(|conv| {
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
                                        //state.write().dispatch(Actions::HideSidebar(true));
                                        if *active_chat != Some(uuid) {
                                            state.write().dispatch(Actions::ShowConversation(conversation_info.conversation.id()));
                                            active_chat.set(Some(uuid));
                                        }
                                    }
                                })
                            }
                        )
                    }
                )
            }
            else { rsx!( SkeletalChats{} ) },
            Nav {
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
            }
        }
    })
}
