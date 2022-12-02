use dioxus::prelude::*;
use uuid::Uuid;
use warp::crypto::DID;

use crate::iutils;
use crate::Account;
use ui_kit::profile_picture::PFP;

#[derive(Props, PartialEq)]
pub struct Props {
    message_id: Uuid,
    message: String,
    is_remote: bool,
    account: Account,
    sender: DID,
    attachments_len: usize,
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
        iutils::get_pfp_from_did(cx.props.sender.clone(), &cx.props.account.clone());

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
                div {
                    class: "reply-message-container",
                    onclick: move |_e| {
                        use_eval(&cx)(format!("
                            document.getElementById('{}-message').scrollIntoView({{
                                behavior: 'smooth',
                                block: 'start'
                            }})
                        ", cx.props.message_id));
                    },
                    if cx.props.attachments_len > 0 {
                        rsx!(div {
                            class: "reply-attachments",
                            match cx.props.attachments_len {
                                1 => rsx!(span {
                                    class: "reply-attachments-count",
                                    "1 attachment"
                                }),
                                _ => rsx!(span {
                                    class: "reply-attachments-count",
                                    "{cx.props.attachments_len} attachments"
                                })
                            }
                        })
                    } else {
                        rsx!(Fragment {})
                    }
                    if !cx.props.message.is_empty() {
                        rsx!(p {
                            "{cx.props.message}",
                        })
                    } else {
                        rsx!(Fragment {})
                    }
                }
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
