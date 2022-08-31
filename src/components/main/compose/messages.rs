use dioxus::prelude::*;
use sir::global_css;
use uuid::Uuid;
use warp::raygun::{Conversation, MessageOptions};

use crate::RAYGUN;


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

    let conversation_id = cx.props.conversation.id();

    let _show_skeleton = conversation_id == Uuid::default();
    
    // Load Multipass & Raygun's Atom Ref
    let raygun = use_atom_ref(&cx, RAYGUN);

    // Read their values from locks
    let rg = raygun.read().clone().unwrap().clone();

    let messages = use_future(&cx, (), |_| async move {
        rg
            .write()
            .get_messages(conversation_id, MessageOptions::default()).await
    });

    let element = cx.render(
        match messages.value() {
            Some(Ok(list)) => rsx! {
                div {
                    class: "messages",
                    list.iter().map(|message| rsx!(
                        div {
                            class: "message",
                            "message"
                        }
                    ))
                }
            },
            Some(Err(_)) => rsx!(div{}),
            None => rsx!(div{}),
        }
    );

    messages.restart();

    element
}