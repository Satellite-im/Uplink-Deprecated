use std::time::Duration;

use dioxus::prelude::*;
use sir::global_css;
use uuid::Uuid;
use warp::raygun::{Conversation, MessageOptions};

use crate::{RAYGUN, STATE};

#[derive(PartialEq, Props)]
pub struct Props {
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    global_css!(
        "
        .messages {
            background: red;
            
        }
    "
    );

    let state = use_atom_ref(&cx, STATE);

    let conversation_id = cx.props.conversation.id();

    let _show_skeleton = conversation_id == Uuid::default();

    // Load Multipass & Raygun's Atom Ref
    let raygun = use_atom_ref(&cx, RAYGUN);

    // Read their values from locks
    let rg = raygun.read().clone().unwrap().clone();

    let messages = use_future(&cx, (), |_| async move {
        rg.write()
            .get_messages(conversation_id, MessageOptions::default())
            .await
    });

    let element = cx.render(match messages.value() {
        Some(Ok(list)) => {
            rsx! {
                div {
                    class: "messages",
                    list.iter().map(|message| message.value().join("\n")).map(|message|{
                        rsx!(
                            div {
                                class: "message",
                                "{message}"
                            }
                        )
                    })
                }
            }
        }
        Some(Err(_e)) => {
            rsx!(div {})
        }
        None => rsx!(div {}),
    });

    messages.restart();

    element
}
