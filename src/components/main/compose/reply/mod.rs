use dioxus::prelude::*;
use warp::crypto::DID;

use crate::utils_internal;
use crate::Account;
use ui_kit::profile_picture::PFP;

#[derive(Props, PartialEq)]
pub struct Props {
    message: String,
    is_remote: bool,
    account: Account,
    sender: DID,
}

#[allow(non_snake_case)]
pub fn Reply(cx: Scope<Props>) -> Element {
    log::debug!("rendering compose/Reply");
    let class = if cx.props.is_remote {
        "remote"
    } else {
        "local"
    };

    let profile_picture =
        utils_internal::get_pfp_from_did(cx.props.sender.clone(), &cx.props.account.clone());

    #[allow(unused_variables)]
    let box_right = "🭽";
    #[allow(unused_variables)]
    let box_left = "🭾";

    #[cfg(target_os = "macos")]
    let box_left = "⎤";

    #[cfg(target_os = "windows")]
    let box_left = "⎤";

    #[cfg(target_os = "macos")]
    let box_right = "⎡";

    #[cfg(target_os = "windows")]
    let box_right = "⎡";

    cx.render({
        rsx! {
            div {
                class: "reply {class}",
                if cx.props.is_remote {
                    rsx!(p {
                            class: "box-drawing left",
                            "{box_right}"
                    })
                } else {
                    let profile_picture = profile_picture.clone();
                    rsx!(PFP {
                        src: profile_picture,
                        size: ui_kit::profile_picture::Size::Normal
                    })
                }
                p {
                    "{cx.props.message}",
                },
                if cx.props.is_remote {
                    rsx!(PFP {
                        src: profile_picture,
                        size: ui_kit::profile_picture::Size::Small
                    })
                } else {
                    rsx!(span {
                        class: "box-drawing",
                        "{box_left}"
                    })
                }
            }
        }
    })
}
