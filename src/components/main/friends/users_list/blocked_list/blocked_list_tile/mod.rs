use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::{
    activity_indicator::ActivityIndicator,
    button::Button,
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use warp::crypto::DID;

use crate::iutils;
use utils::Account;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    friend: DID,
    friend_username: String,
}

#[allow(non_snake_case)]
pub fn BlockedListTile(cx: Scope<Props>) -> Element {
    log::debug!("rendering Blocked Users");

    let mp = cx.props.account.clone();

    let username = cx.props.friend_username.clone();
    let show_skeleton = username.is_empty();

    let profile_picture = iutils::get_pfp_from_did(cx.props.friend.clone(), &mp);

    cx.render(rsx! {
        div {
            class: "friend",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                rsx!(PFP {
                    src: profile_picture,
                    size: ui_kit::profile_picture::Size::Normal
                })
            )},
            div {
                class: "who",
                if show_skeleton {rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    h3 {
                        "{username}"
                    },
                    ActivityIndicator {
                        inline: true,
                        remote_did: cx.props.friend.clone(),
                        account: cx.props.account.clone(),
                    }
                )}
            },
            div {
                class: "request-controls",
                div {
                    class: "control-wrap",
                    if show_skeleton {rsx!(
                        Button {
                            icon: Shape::ChatBubbleBottomCenterText,
                            disabled: true,
                            on_pressed: move |_| {}
                        }
                    )} else {rsx!(
                        Button {
                            text:"Unblock".to_string(),
                            state: ui_kit::button::State::Secondary,
                            on_pressed: move |_| {
                                let mut multipass = cx.props.account.clone();
                                let did_to_unblock = cx.props.friend.clone();
                                match multipass.unblock(&did_to_unblock) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        log::debug!("faied to unblock friend {}:{}", &cx.props.friend, e);
                                    }
                                }
                            }
                        },
                    )}
                }
            }
        }
    })
}
