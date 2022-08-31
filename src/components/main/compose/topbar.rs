use dioxus::prelude::*;
use sir::global_css;
use warp::crypto::DID;

use crate::{
    components::ui_kit::skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    state::Conversation,
    MULTIPASS,
};

#[derive(Props)]
pub struct Props<'a> {
    conversation: Conversation,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    global_css!(
        "
        .topbar {
            height: 40px;
            display: inline-flex;
            flex-direction: row;
            padding: 1rem;

            .pfp {
                height: 40px;
                width: 40px;
                border-radius: 20px;
                background: var(--theme-text-muted);
            }
            .who {
                flex: 1;
                heigth: 40px;
                text-align: left;
                padding: 0 1rem;
                display: inline-flex;
                flex-direction: column;

                .top-row {
                    display: inline-flex;
                    flex-direction: row;
                    h3 {
                        margin: 0;
                        font-size: 13pt;
                    }
                }

                span {
                    font-size: 12px;
                    color: var(--theme-text-darker);
                }
            }
            
        }
    "
    );

    let multipass = use_atom_ref(&cx, MULTIPASS);
    let mp = multipass.read().clone().unwrap().clone();

    let ident = mp
        .read()
        .get_own_identity()
        .expect("Unexpected error <temp>");

    let chatting_with = cx
        .props
        .conversation
        .recipients
        .iter()
        .filter(|did| ident.did_key().ne(did))
        .last()
        .expect("blah");

    let user = match mp.read().get_identity(chatting_with.clone().into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    let username = user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());

    let status = match user
        .first()
        .map(|i| i.status_message())
        .unwrap_or_else(|| None)
    {
        Some(s) => s,
        None => String::from(""),
    };

    let show_skeleton = username.is_empty();

    cx.render(rsx! {
        div {
            class: "topbar",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                div {
                    class: "pfp"
                },
            )},
            div {
                class: "who",
                div {
                    class: "top-row",
                    if show_skeleton{rsx!(
                        InlineSkeleton {}
                    )} else {rsx!(
                        h3 {
                            "{username}"
                        }
                    )}
                },
                if show_skeleton{rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    span {
                        "{status}"
                    }
                )}
            }
        },
    })
}
