use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::iutils;
use utils::Account;

use ui_kit::{
    button::{self, Button},
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use warp::multipass::identity::FriendRequest;

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
    log::debug!("rendering FriendRequest");
    let mp = cx.props.account.clone();

    let did = if cx.props.deny_only {
        cx.props.request.to()
    } else {
        cx.props.request.from()
    };

    let username = iutils::get_username_from_did(did.clone(), &mp);
    let show_skeleton = username.is_empty();
    let profile_picture = iutils::get_pfp_from_did(did, &mp);

    cx.render(rsx! {
        div {
            class: "request",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                rsx!(PFP {
                    src: profile_picture,
                    size: ui_kit::profile_picture::Size::Normal
                })
             )}
            div {
                class: "who",
                if show_skeleton {rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    h3 {
                        "{username}",
                    }
                )}
            },
            div {
                class: "request-controls",
                div {
                    class: "control-wrap",
                    rsx!(
                        Button {
                            icon: Shape::XMark,
                            state: button::State::Secondary,
                            on_pressed: move |_| {
                                cx.props.on_deny.call(());
                            }
                        }
                    )
                }
                (!cx.props.deny_only).then(|| rsx!{
                    if show_skeleton {rsx!(
                        div {
                            class: "control-wrap",
                            Button {
                                icon: Shape::Check,
                                state: button::State::Primary,
                                disabled: true,
                                on_pressed: move |_| {}
                            }
                        }
                    )} else {rsx!(
                        div {
                            class: "control-wrap",
                            Button {
                                icon: Shape::Check,
                                state: button::State::Primary,
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
