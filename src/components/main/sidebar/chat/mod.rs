use crate::{
    components::ui_kit::skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    Account, CONVERSATIONS, CONVERSATION_METADATA, LANGUAGE,
};
use dioxus::prelude::*;
use warp::raygun::Conversation;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    conversation: Conversation,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Chat<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let conversations = use_atom_ref(&cx, CONVERSATIONS);
    let chats_meta = use_atom_ref(&cx, CONVERSATION_METADATA);
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let mp = cx.props.account.clone();

    let ident = mp
        .read()
        .get_own_identity()
        .expect("Unexpected error <temp>");

    let username = cx
        .props
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

    let num_unread = chats_meta
        .read()
        .v
        .get(&cx.props.conversation.id())
        .map(|ci| ci.total_messages - ci.last_read)
        .and_then(|x| if x > 0 { Some(x) } else { None });

    let active = match &conversations.read().current_chat {
        Some(active_chat) => {
            if active_chat.id() == cx.props.conversation.id() {
                "active"
            } else {
                "none"
            }
        }
        None => "",
    };

    if show_skeleton {
        cx.render(rsx! {
            div {
                class: "chat {active}",
                onclick: move |_| cx.props.on_pressed.call(()),
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
                onclick: move |_| cx.props.on_pressed.call(()),
                div {
                    class: "pfp"
                },
                div {
                    class: "who",
                    div {
                        class: "top-row",
                        h3 {
                            "{username}"
                        },
                        span {
                            class: "timestamp",
                            "10:00am"
                        }
                    },
                    span {
                        match num_unread {
                            Some(unread) => rsx!("unread: {unread}"),
                            None => rsx!("{l.chat_placeholder}")
                        }
                    }
                }
            }
        })
    }
}
