use dioxus::prelude::*;
use warp::raygun::Conversation;
use crate::{
    components::ui_kit::skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    LANGUAGE, STATE, Account,
};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    conversation: Conversation,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Chat<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);
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

    let active = match state.read().chat.clone() {
        Some(active_chat) => {
            if active_chat.id() == cx.props.conversation.id() {
                "active"
            } else {
                "none"
            }
        }
        None => "",
    };

    cx.render(rsx! {
        div {
            class: "chat {active}",
            onclick: move |_| cx.props.on_pressed.call(()),
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                div {
                    class: "pfp"
                },
            )},
            div {
                class: "who",
                if show_skeleton {rsx!(
                    InlineSkeleton {},
                    InlineSkeleton {}
                )} else {rsx!(
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
                        "{l.chat_placeholder}"
                    }
                )}
            },
        }
    })
}
