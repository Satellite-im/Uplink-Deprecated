use dioxus::prelude::*;
use uuid::Uuid;
use warp::{
    crypto::DID,
    raygun::{Conversation, MessageOptions},
};

use crate::{components::main::compose::msg::Msg, Account, Messaging};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    let conversation_id = cx.props.conversation.id();

    let _show_skeleton = conversation_id == Uuid::default();

    // Load Multipass & Raygun's Atom Ref
    let multipass = cx.props.account.clone();
    let raygun = cx.props.messaging.clone();

    // Read their values from locks
    let rg = raygun.clone();
    let mp = multipass.clone();

    let messages = use_future(&cx, (), |_| async move {
        rg.write()
            .get_messages(conversation_id, MessageOptions::default().set_range(0..64))
            .await
    });

    let ident = mp
        .read()
        .get_own_identity()
        .map(|id| id.did_key())
        .unwrap_or_default();

    let element = cx.render(match messages.value() {
        Some(Ok(list)) => {
            let mut prev_sender = "".to_string();
            rsx! {
                div {
                    class: "messages",
                    list.iter().rev().peekable().map(|message| {

                        let msg_sender = message.sender().to_string();
                        let i = ident.to_string();
                        let remote = i != msg_sender;
                        let last = prev_sender != msg_sender;
                        let middle = prev_sender == msg_sender;
                        let first = false;

                        prev_sender = message.sender().to_string();

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
