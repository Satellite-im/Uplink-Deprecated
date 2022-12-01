use std::time::{Duration, Instant};

use dioxus::prelude::*;
use futures::StreamExt;
use uuid::Uuid;
use warp::raygun::MessageEvent;

use crate::{Messaging, STATE};

// the local side will send an event to indicate typing and refresh it periodically
// the remote side will disable the typing indicator if a refresh isn't received in time
// the remote side will also disable the typing indicator if a new message is received
enum ChanCmd {
    Typing { chat_id: Uuid, rg: Messaging },
    NotTyping,
    RefreshTyping { rg: Messaging },
}

#[derive(Props)]
pub struct Props<'a> {
    messaging: Messaging,
    on_input: EventHandler<'a, String>,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    placeholder: String,
}

#[allow(non_snake_case)]
#[allow(clippy::clone_double_ref)]
#[allow(unused_assignments)]
pub fn TextArea<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering reusable/textarea");

    let state = use_atom_ref(&cx, STATE);
    let current_chat = state.read().selected_chat;
    // send typing indicators periodically
    let chan = use_coroutine(&cx, |mut rx: UnboundedReceiver<ChanCmd>| async move {
        // (conversation ID, time of last keystroke)
        let mut typing_state: Option<(Uuid, Instant)> = None;

        while let Some(cmd) = rx.next().await {
            match cmd {
                ChanCmd::Typing { chat_id, mut rg } => {
                    log::debug!("typing indicator tx: received typing ");
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
                    log::debug!("typing indicator tx: received not typing ");
                    typing_state = None;
                }
                ChanCmd::RefreshTyping { mut rg } => {
                    log::debug!("typing indicator tx: received refresh typing ");
                    if let Some((chat_id, last_time)) = typing_state {
                        let elapsed = Instant::now().duration_since(last_time);
                        if elapsed > Duration::from_secs(3) {
                            typing_state = None;
                        } else if let Err(_) = rg.send_event(chat_id, MessageEvent::Typing).await {
                            // todo: log error
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
            log::debug!("send typing indicator refresh");
            tokio::time::sleep(Duration::from_secs(3)).await;
            chan1.send(ChanCmd::RefreshTyping { rg: rg.clone() });
        }
    });
    let chan2 = chan.clone();
    let chan3 = chan.clone();
    cx.render(rsx!(ui_kit::textarea::TextArea {
        on_input: move |val: String| {
            let chat_id = match current_chat {
                Some(c) => c,
                None => {
                    cx.props.on_input.call(val);
                    return;
                }
            };
            chan2.send(ChanCmd::Typing {
                chat_id,
                rg: cx.props.messaging.clone(),
            });
            cx.props.on_input.call(val);
        },
        on_submit: move |val: String| {
            chan3.send(ChanCmd::NotTyping);
            cx.props.on_submit.call(val);
        },
        text: cx.props.text.clone(),
        placeholder: cx.props.placeholder.clone(),
    }))
}
