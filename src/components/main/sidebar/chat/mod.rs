use crate::{
    components::reusable::context_menu::{ContextItem, ContextMenu},
    iutils,
    state::{Actions, ConversationInfo, LastMsgSent},
    Account, Messaging, LANGUAGE, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::stream::StreamExt;
use ui_kit::{
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use uuid::Uuid;
use warp::crypto::DID;
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
    log::debug!("rendering main/sidebar/Chat");
    let state = use_atom_ref(&cx, STATE).clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    // must be 'moved' into the use_future. don't pass it as a dependency because that won't work with
    // Rust's ownership model
    let unread_count = use_state(&cx, || 0_u32).clone();
    // need this one for display
    let unread_count2 = unread_count.clone();
    let online_status = use_state(&cx, || IdentityStatus::Offline).clone();
    let online_status2 = online_status.clone();

    let last_msg_time = cx
        .props
        .last_msg_sent
        .clone()
        .map(|x| iutils::display_msg_time(x.time));
    let last_msg_sent = cx.props.last_msg_sent.clone().map(|x| x.value);
    let tx_chan = cx.props.tx_chan.clone();

    let mut rg = cx.props.messaging.clone();
    let mp = cx.props.account.clone();

    let ident = mp
        .read()
        .get_own_identity()
        .expect("Unexpected error <temp>");

    let did = cx
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
        .map(|i| i.did_key())
        .last()
        .unwrap_or_default();

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
                        log::debug!("updating online_status ");
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
                if *unread_count.current() != 0 {
                    log::debug!("sidebar/chat exiting future ");
                    unread_count.set(0);
                }
                // very important: don't open two message streams - if this is the active chat, the messages Element will read the stream and this
                // chat component shouldn't.
                return;
            }

            let num_unread_messages = conversation_info.num_unread_messages;
            if *unread_count.current() != num_unread_messages {
                log::debug!("updating num_unread_messages ");
                unread_count.set(num_unread_messages);
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
                    match rg.get_message(conversation_id, message_id).await {
                        Ok(msg) => {
                            log::debug!("sidebar/chat streamed a message");
                            tx_chan.send(msg.clone());
                            unread_count.modify(|x| x + 1);
                            // will silently remain zero if you only use *unread_count
                            conversation_info.num_unread_messages = *unread_count.current();
                            conversation_info.last_msg_sent = Some(LastMsgSent::new(&msg.value()));
                            state
                                .write()
                                .dispatch(Actions::UpdateConversation(conversation_info.clone()));
                        }
                        Err(_e) => {
                            // todo: possibly log errorv
                        }
                    };
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
                id: "chat-{did}",
                onclick: move |_| {
                    cx.props.on_pressed.call(cx.props.conversation_info.conversation.id());
                },
                ContextMenu {
                    parent: format!("chat-{}", &did),
                    items: cx.render(rsx! {
                        ContextItem {
                            icon: Shape::EyeOff,
                            onpressed: move |_| {},
                            text: String::from("Mark Seen"),
                        },
                        hr{}
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Call"),
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Share File"),
                        },
                        hr{}
                        ContextItem {
                            icon: Shape::X,
                            onpressed: move |_| {},
                            text: String::from("Remove Chat"),
                        },
                        ContextItem {
                            danger: true,
                            icon: Shape::Ban,
                            onpressed: move |_| {},
                            text: String::from("Block User"),
                        },
                    })
                },
                ChatPfp {status: online_status2, account: cx.props.account.clone(), did: did },
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
                        match *unread_count2.current() {
                            0 =>  rsx!( div {
                                class: "unread-placeholder",
                            }),
                            _ => rsx!( div {
                                class: "unread-count",
                                span {
                                    "{unread_count2}"
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
pub fn ChatPfp(cx: Scope, status: UseState<IdentityStatus>, account: Account, did: DID) -> Element {
    let is_online = match *status.current() {
        IdentityStatus::Online => "online",
        _ => "",
    };
    let profile_picture = iutils::get_pfp_from_did(did.clone(), account);

    cx.render(rsx! {
        div {
            class: "pfp-container",
            PFP {
                src: profile_picture,
                size: ui_kit::profile_picture::Size::Normal
            },
            div {
                class: "pfs {is_online}"
            }
        }
    })
}
