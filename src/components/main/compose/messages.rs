use dioxus::prelude::*;
use sir::global_css;
use uuid::Uuid;
use warp::{raygun::{Conversation, MessageOptions}, crypto::DID};

use crate::{RAYGUN, components::main::compose::msg::Msg, MULTIPASS};

#[derive(PartialEq, Props)]
pub struct Props {
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    global_css!("
        .messages {
            display: inline-flex;
            flex-direction: column-reverse;
            width: calc(100% - 2rem);
            padding: 0 1rem;
            height: 100%;
            overflow-y: scroll;
        }
    ");

    let conversation_id = cx.props.conversation.id();

    let _show_skeleton = conversation_id == Uuid::default();

    // Load Multipass & Raygun's Atom Ref
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let raygun = use_atom_ref(&cx, RAYGUN);

    // Read their values from locks
    let rg = raygun.read().clone().unwrap().clone();
    let mp = multipass.read().clone().unwrap().clone();

    let messages = use_future(&cx, (), |_| async move {
        rg.write()
            .get_messages(conversation_id, MessageOptions::default())
            .await
    });

    
    let element = cx.render(match messages.value() {
        Some(Ok(list)) => {
            let mut prev_sender = "".to_string();
            rsx! {
                div {
                    class: "messages",
                    list.iter().rev().peekable().map(|message|{
                        let ident = match mp
                            .read()
                            .get_own_identity()
                            {
                                Ok(id) => id.did_key(),
                                Err(_) => DID::default(),
                            };
                        
                        
                        let msg_sender = message.clone().sender().to_string();
                        let i = ident.to_string();
                        let remote = i != msg_sender;
                        let last = prev_sender != msg_sender;
                        let middle = prev_sender == msg_sender;
                        let first = false;

                        prev_sender = message.clone().sender().to_string();

                        rsx!(
                            Msg {
                                message: message.clone(),
                                remote: remote,
                                last: last,
                                first: first,
                                middle: middle,
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
