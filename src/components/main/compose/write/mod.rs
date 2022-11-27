use std::time::{Duration, Instant};

use crate::{iutils::config::Config, Messaging, LANGUAGE, STATE};
use audio_factory::AudioFactory;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use ui_kit::{
    context_menu::{ContextItem, ContextMenu},
    icon_button::{self, IconButton},
    small_extension_placeholder::SmallExtensionPlaceholder,
    textarea::TextArea,
};
use utils::extensions::BasicExtension;
use uuid::Uuid;
use warp::raygun::{MessageEvent, RayGunEvents};

#[derive(Props)]
pub struct Props<'a> {
    messaging: Messaging,
    on_submit: EventHandler<'a, String>,
    on_upload: EventHandler<'a, ()>,
}

// the local side will send an event to indicate typing and refresh it periodically
// the remote side will disable the typing indicator if a refresh isn't received in time
// the remote side will also disable the typing indicator if a new message is received
enum ChanCmd {
    Typing { chat_id: Uuid, rg: Messaging },
    NotTyping,
    RefreshTyping { rg: Messaging },
}

#[allow(non_snake_case)]
pub fn Write<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering compose/Write");
    let config = Config::load_config_or_default();

    let state = use_atom_ref(&cx, STATE);
    let current_chat = state.read().current_chat;
    let text = use_state(&cx, String::new);
    let l = use_atom_ref(&cx, LANGUAGE).read();

    // send typing indicators periodically
    let chan = use_coroutine(&cx, |mut rx: UnboundedReceiver<ChanCmd>| async move {
        // (conversation ID, time of last keystroke)
        let mut typing_state: Option<(Uuid, Instant)> = None;

        while let Some(cmd) = rx.next().await {
            match cmd {
                ChanCmd::Typing { chat_id, mut rg } => {
                    let chat_id_changed = match typing_state {
                        None => true,
                        Some((prev_id, _)) => prev_id != chat_id,
                    };

                    typing_state = Some((chat_id, Instant::now()));

                    if chat_id_changed {
                        if let Err(_) = rg.send_event(chat_id, MessageEvent::Typing).await {
                            // todo: log error
                        }
                    }
                }
                ChanCmd::NotTyping => {
                    typing_state = None;
                }
                ChanCmd::RefreshTyping { mut rg } => {
                    if let Some((chat_id, last_time)) = typing_state {
                        let elapsed = Instant::now().duration_since(last_time);
                        if elapsed > Duration::from_secs(3) {
                            typing_state = None;
                        } else {
                            if let Err(_) = rg.send_event(chat_id, MessageEvent::Typing).await {
                                // todo: log error
                            }
                        }
                    }
                }
            }
        }
    });

    // periodically re-send typing indicator
    let chan1 = chan.clone();
    use_future(&cx, &cx.props.messaging.clone(), |rg| async move {
        loop {
            // todo: start this wait when a user changes from not typing to typing
            let _ = tokio::time::sleep(Duration::from_secs(3));
            chan1.send(ChanCmd::RefreshTyping { rg: rg.clone() });
        }
    });

    let chan2 = chan.clone();
    let chan3 = chan.clone();
    cx.render(rsx! {
        div {
            class: "write",
            id: "write",
            ContextMenu {
                parent: String::from("write"),
                items: cx.render(rsx! {
                    ContextItem {
                        onpressed: move |_| {},
                        icon: Shape::Clipboard,
                        text: String::from("Copy Conversation ID")
                    },
                })
            },
            IconButton {
                icon: Shape::Plus,
                on_pressed: move |_| {
                    let _ = &cx.props.on_upload.call(());
                },
            },
            TextArea {
                on_input: move |_e| {
                    let chat_id = match current_chat {
                        Some(c) => c,
                        None => return
                    };
                    chan2.send(ChanCmd::Typing{chat_id, rg: cx.props.messaging.clone()});
                }
                on_submit: move |val| {
                    chan3.send(ChanCmd::NotTyping);
                    cx.props.on_submit.call(val);
                },
                text: text.clone(),
                placeholder: l.chatbar_placeholder.to_string()
            }
            config.developer.developer_mode.then(|| rsx! {
                div {
                    class: "extension-holder",
                    SmallExtensionPlaceholder {}
                }
            })
            div {
                class: "chatbar_extensions",
                AudioFactory::render()
            },
            div {
                id: "send",
                IconButton {
                    icon: Shape::ArrowRight,
                    state: icon_button::State::Secondary,
                    on_pressed: move |_| {
                        let text = text.clone();
                        let _ = &cx.props.on_submit.call(text.to_string());
                        text.set(String::from(""));
                    },
                }
            }
        }
    })
}
