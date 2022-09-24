use crate::{components::main::compose::msg::Msg, Account, Messaging};
use dioxus::prelude::*;
use uuid::Uuid;
use warp::{
    crypto::DID,
    raygun::{Conversation, MessageOptions},
};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    let conversation_id = cx.props.conversation.id();

    // let _show_skeleton = conversation_id == Uuid::default();

    // Read their values from locks
    let rg = cx.props.messaging.clone();
    let mp = cx.props.account.clone();

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
