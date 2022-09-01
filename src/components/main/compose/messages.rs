use dioxus::prelude::*;
use sir::global_css;
use uuid::Uuid;
use warp::{raygun::{Conversation, MessageOptions}, crypto::DID};

use crate::{RAYGUN, components::main::compose::msg::Msg, MULTIPASS, STATE};

#[derive(PartialEq, Props)]
pub struct Props {
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    global_css!("
        .messages {
            display: inline-flex;
            flex-direction: column;
            width: calc(100% - 2rem);
            padding: 0 1rem;
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
            rsx! {
                div {
                    class: "messages",
                    list.iter().map(|message|{
                        let ident = match mp
                            .read()
                            .get_own_identity()
                            {
                                Ok(id) => id.did_key(),
                                Err(_) => DID::default(),
                            };
                        
                        let remote = ident.to_string() != message.clone().sender().to_string();
                        rsx!(
                            Msg {
                                message: message.clone(),
                                remote: remote,
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
