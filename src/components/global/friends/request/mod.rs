use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use fermi::prelude::*;
use warp::multipass::identity::FriendRequest;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    }, Account,
};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    request: FriendRequest,
    deny_only: bool,
    on_deny: EventHandler<'a, ()>,
    on_accept: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn FriendRequest<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let multipass = cx.props.account.clone();
    let mp = multipass.clone();

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
                                cx.props.on_deny.call(());
                            }
                        }
                    )}
                }
                (!cx.props.deny_only).then(|| rsx!{
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
                                    cx.props.on_accept.call(());
                                }
                            }
                        }
                    )}
                })
            }
        }
    })
}
