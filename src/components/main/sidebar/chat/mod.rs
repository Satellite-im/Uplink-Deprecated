use crate::{
    components::ui_kit::skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    state::{Actions, ConversationInfo, LastMsgSent},
    Account, Messaging, LANGUAGE, STATE,
};
use dioxus::prelude::*;
use futures::stream::StreamExt;
use uuid::Uuid;
use warp::multipass::{identity::IdentityStatus, IdentityInformation};
use warp::raygun::{Message, MessageEventKind, RayGun, RayGunStream};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    conversation_info: ConversationInfo,
    messaging: Messaging,
    #[props(!optional)]
    last_msg_sent: Option<LastMsgSent>,
    is_active: bool,
    // used to send received messages to the Sidebar so they can be used to create a notification
    tx_chan: CoroutineHandle<Message>,
    on_pressed: EventHandler<'a, Uuid>,
}

#[allow(non_snake_case)]
pub fn Chat<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE).clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let online_status = use_state(&cx, || IdentityStatus::Offline).clone();
    let online_status2 = online_status.clone();

    let last_msg_time = cx.props.last_msg_sent.clone().map(|x| x.display_time());
    let last_msg_sent = cx.props.last_msg_sent.clone().map(|x| x.value);
    let num_unread = cx.props.conversation_info.num_unread_messages;
    let tx_chan = cx.props.tx_chan.clone();

    let mut rg = cx.props.messaging.clone();
    let mp = cx.props.account.clone();

    let ident = mp
        .read()
        .get_own_identity()
        .expect("Unexpected error <temp>");

    let username = cx
        .props
        .conversation_info
        .conversation
        .recipients()
        .iter()
        //filters out our own did key in the iter
        .filter(|did| ident.did_key().ne(did))
        //tries get_identity so if it returns Option::Some it would be the map item, otherwise its filtered out
        .filter_map(|did| mp.read().get_identity(did.clone().into()).ok())
        //flatted the nested iterators
        .flatten()
        .map(|i| i.username())
        .last()
        .unwrap_or_default();

    let show_skeleton = username.is_empty();
    let active = if cx.props.is_active { "active" } else { "none" };

    use_future(
        &cx,
        (
            &cx.props.conversation_info.clone(),
            &cx.props.account.clone(),
        ),
        |(conversation_info, account)| async move {
            let remote_did = conversation_info
                .conversation
                .recipients()
                .last()
                .cloned()
                .unwrap_or_default();

            loop {
                if let Ok(current_status) = account.identity_status(&remote_did) {
                    if *online_status.current() != current_status {
                        online_status.set(current_status);
                    }
                }
                // todo: consider making this configurable or longer
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            }
        },
    );

    use_future(
        &cx,
        (&cx.props.conversation_info.clone(), &cx.props.is_active),
        |(mut conversation_info, is_active)| async move {
            if is_active {
                return;
            }

            let mut stream = loop {
                match rg
                    .get_conversation_stream(conversation_info.conversation.id())
                    .await
                {
                    Ok(stream) => break stream,
                    Err(e) => match &e {
                        warp::error::Error::RayGunExtensionUnavailable => {
                            //Give sometime for everything in the background to fully line up
                            //Note, if this error still happens, it means there is an fatal error
                            //      in the background
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        _ => {
                            // todo: properly report this error
                            // eprintln!("failed to get_conversation_stream: {}", e);
                            tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                        }
                    },
                }
            };

            while let Some(event) = stream.next().await {
                if let MessageEventKind::MessageReceived {
                    conversation_id,
                    message_id,
                } = event
                {
                    let msg = match rg.get_message(conversation_id, message_id).await {
                        Ok(msg) => {
                            // ensure there's a notification.
                            tx_chan.send(msg.clone());
                            msg.value()
                                .iter()
                                .take(2)
                                .cloned()
                                .collect::<Vec<String>>()
                                .join("\n")
                        }
                        Err(_) => {
                            // todo: handle error
                            "".to_string()
                        }
                    };

                    conversation_info.num_unread_messages += 1;
                    conversation_info.last_msg_sent = Some(LastMsgSent::new(msg));
                    state
                        .write()
                        .dispatch(Actions::UpdateConversation(conversation_info.clone()));
                }
            }
        },
    );

    if show_skeleton {
        cx.render(rsx! {
            div {
                class: "chat {active}",
                onclick: move |_|{
                    cx.props.on_pressed.call(cx.props.conversation_info.conversation.id());
                } ,
                PFPSkeleton {},
                div {
                    class: "who",
                    InlineSkeleton {},
                    InlineSkeleton {}
                }
            }
        })
    } else {
        cx.render(rsx! {
            div {
                class: "chat {active}",
                onclick: move |_| {
                    cx.props.on_pressed.call(cx.props.conversation_info.conversation.id());
                },
                ChatPfp {status: online_status2},
                div {
                    class: "who",
                    div {
                        class: "top-row",
                        h3 {
                            "{username}"
                        },
                        last_msg_time.map(|time| {
                            rsx! (
                                span {
                                    class: "timestamp",
                                   "{time}"
                                }
                            )
                        }),
                    },
                    div {
                        class: "msg-container",
                        span {
                            class: "block-with-text",
                            match last_msg_sent {
                                Some(msg) => rsx!(p {
                                    "{msg}"
                                }),
                                None => rsx!(p {
                                    "{l.chat_placeholder}"
                                })
                            }
                        }
                        match num_unread {
                            0 =>  rsx!( div {
                                class: "unread-placeholder",
                            }),
                            _ => rsx!( div {
                                class: "unread-count",
                                span {
                                    "{num_unread}"
                                }
                            }),
                        }
                    }
                }
            }
        })
    }
}

#[inline_props]
#[allow(non_snake_case)]
pub fn ChatPfp(cx: Scope, status: UseState<IdentityStatus>) -> Element {
    let is_online = match *status.current() {
        IdentityStatus::Online => "online",
        _ => "",
    };
    cx.render(rsx! {
        div {
            class: "pfp-container",

            div {
                class: "pfp"
            },
            div {
                class: "pfs {is_online}"
            }
        }
    })
}
