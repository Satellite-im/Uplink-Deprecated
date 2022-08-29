use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use warp::multipass::identity::FriendRequest;

use crate::{
    components::ui_kit::{icon_button::{self, IconButton}, skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton}},
    MULTIPASS,
};

#[derive(Props)]
pub struct Props<'a> {
    request: FriendRequest,
    deny_only: bool,
    on_deny: EventHandler<'a, ()>,
    on_accept: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn FriendRequest<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let mp = multipass.read().clone().unwrap().clone();

    let did = if cx.props.deny_only {
        cx.props.request.to()
    } else {
        cx.props.request.from()
    };

    let user = match mp.read().get_identity(did.clone().into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    let username = user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());

    let show_skeleton = username.is_empty();

    global_css! {"
        .request {
            display: inline-flex;
            flex-direction: row;
            align-items: center;
            width: 100%;

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

                h3 {
                    margin: 0;
                    font-size: 13px;
                }
            }

            .request-controls {
                height: 40px;
                display: inline-flex;

                .control-wrap {
                    margin-left: 1rem;
                }
            }
        }
    "};

    cx.render(rsx! {
        div {
            class: "request",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                div {
                    class: "pfp"
                    
                },
            )}
            div {
                class: "who",
                if show_skeleton {rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    h3 {
                        "{username}"
                    }
                )}
            },
            div {
                class: "request-controls",
                div {
                    class: "control-wrap",
                    if show_skeleton {rsx!(
                        IconButton {
                            icon: Shape::X,
                            state: icon_button::State::Secondary,
                            disabled: true,
                            on_pressed: move |_| {}
                        }
                    )} else {rsx!(
                        IconButton {
                            icon: Shape::X,
                            state: icon_button::State::Secondary,
                            on_pressed: move |_| {
                                cx.props.on_accept.call(());
                            }
                        }
                    )}
                }
                if cx.props.deny_only {rsx!(
                    span {}
                )} else {
                    if show_skeleton {rsx!(
                        div {
                            class: "control-wrap",
                            IconButton {
                                icon: Shape::Check,
                                state: icon_button::State::Primary,
                                disabled: true,
                                on_pressed: move |_| {}
                            }
                        }
                    )} else {rsx!(
                        div {
                            class: "control-wrap",
                            IconButton {
                                icon: Shape::Check,
                                state: icon_button::State::Primary,
                                on_pressed: move |_| {
                                    cx.props.on_deny.call(());
                                }
                            }
                        }
                    )}
                }
            }
        }
    })
}
