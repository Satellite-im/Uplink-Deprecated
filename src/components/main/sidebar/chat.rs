use dioxus::prelude::*;
use sir::global_css;
use warp::crypto::DID;

use crate::{components::ui_kit::skeletons::{pfp::PFPSkeleton, inline::InlineSkeleton}, MULTIPASS, STATE};

#[derive(Props)]
pub struct Props<'a> {
    did: DID,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Chat<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);

    global_css! {"
        .sidebar {
            .chat {
                display: inline-flex;
                flex-direction: row;
                width: calc(100%);
                padding: 0.5rem;
                margin-left: -0.5rem;
                border-radius: 4px;
                cursor: pointer;
                margin-bottom: 0.5rem;

                &:hover,
                &.active {
                    background: var(--theme-background-light);
                }

                .inline-skeleton {
                    margin-bottom: 0.5rem;
                }

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
                            font-size: 13px;
                        }
                        .timestamp {
                            flex: 1;
                            text-align: right;
                            color: var(--theme-text-muted);
                        }
                    }

                    span {
                        font-size: 12px;
                        color: var(--theme-text-darker);
                    }
                }

                
            }
        }
    "}

    let multipass = use_atom_ref(&cx, MULTIPASS);
    let mp = multipass.read().clone().unwrap().clone();

    let user = match mp.read().get_identity(cx.props.did.clone().into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    let username = user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());

    let show_skeleton = username.is_empty();
    let active = match state.read().chat.clone() {
        Some(active_chat) => {
            if active_chat == cx.props.did {
                "active"
            } else {
                "none"
            }
        },
        None => "",
    };

    cx.render(rsx!{
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
                        "It's quiet... click here to start a convorsation with Jane Doe."
                    }
                )}
            },
        }
    })
}