use crate::{components::main::compose::msg::Msg, Account};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use warp::raygun::Message;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messages: Vec<Message>,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let ident = cx.props.account.read().get_own_identity().unwrap();

    cx.render({
            let mut prev_sender = "".to_string();
            rsx! {
                div {
                    class: "messages",
                    cx.props.messages.iter().rev().map(|message|{
                        let msg_sender = message.sender().to_string();
                        let i = ident.did_key().to_string();
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
                    div {
                        class: "encrypted-notif",
                        Icon {
                            icon: Shape::LockClosed
                        }
                        p {
                            "Messages are encrypted locally, anyone outside of this chat cannot modify, or read them."
                        }
                    }
                }
            }
    })
}
