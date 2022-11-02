use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use uuid::Uuid;
use warp::{multipass::identity::Identity, raygun::Message};

use crate::{
    components::{
        main::{
            friends::Friends,
            profile::Profile,
            sidebar::nav::{Nav, NavEvent},
        },
        ui_kit::{
            button::Button, extension_placeholder::ExtensionPlaceholder, icon_button::IconButton,
            icon_input::IconInput,
        },
    },
    extensions::*,
    state::Actions,
    utils::{config::Config, notifications::PushNotification},
    Account, Messaging, LANGUAGE, STATE,
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
    let config = Config::load_config_or_default();
    let mp = cx.props.account.clone();

    let state = use_atom_ref(&cx, STATE);
    let show_friends = use_state(&cx, || false);
    let show_profile = use_state(&cx, || false);

    let l = use_atom_ref(&cx, LANGUAGE).read();
    let friendString = l.friends.to_string();
    let favString = l.favorites.to_string();
    let newchatdString = l.new_chat.to_string();
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
            // todo: put display_user and display_username into a common library
            let display_user = mp
                .read()
                .get_identity(msg.sender().clone().into())
                .unwrap_or_default();

            let display_username = display_user
                .first()
                .map(Identity::username)
                .unwrap_or_else(String::new);
            PushNotification(display_username, msg.value().join("\n"));
        }
    });

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
            label {
                "{favString}"
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
                        "{newchatdString}"
                    }
                },
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
                            class: "gradient_mask"
                        },
                        div {
                            class: "gradient_mask is_bottom"
                        },
                        div {
                            class: "chats",
                            state.read().all_chats.iter().map(|(key, conv)| {
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
                            on_pressed: move |_| show_friends.set(true),
                        },
                    }
                )
            },
            (**show_friends).then(|| rsx!{
                //TODO: this is a fix for now, but next milestone we should rework popups to
                // de-render themselves after css hide animations are completed.
                Friends {
                    account: cx.props.account.clone(),
                    messaging: cx.props.messaging.clone(),
                    title: friendString,
                    show: true,
                    icon: Shape::Users,
                    on_hide: move |_| {
                        show_friends.set(false);
                    },
                }
            }),
            Profile {
                account: cx.props.account.clone(),
                show: *show_profile.clone(),
                on_hide: move |_| show_profile.set(false),
            },
            Nav {
                account: cx.props.account.clone(),
                on_pressed: move | e: NavEvent | {
                    show_friends.set(false);
                    show_profile.set(false);
                    match e {
                        NavEvent::Home => {
                        },
                        NavEvent::Files => {
                            use_router(&cx).push_route("/main/files", None, None);
                        },
                        NavEvent::Friends => {
                            show_friends.set(true);
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
