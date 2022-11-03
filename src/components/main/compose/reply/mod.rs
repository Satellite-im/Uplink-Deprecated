use dioxus::prelude::*;

use crate::Account;

#[derive(Props, PartialEq)]
pub struct Props {
    message: String,
    is_remote: bool,
    account: Account,
}

#[allow(non_snake_case)]
pub fn Reply(cx: Scope<Props>) -> Element {
    let class = if cx.props.is_remote {
        "remote"
    } else {
        "local"
    };

    let identity = cx.props.account.clone().read().get_own_identity().unwrap();
    let profile_picture = identity.graphics().profile_picture();
    let profile_picture2 = profile_picture.clone();


    cx.render({
        rsx! {
            div {
                class: "reply {class}",
                (cx.props.is_remote).then(|| rsx! {
                    p {
                        class: "box-drawing left",
                        "🭽"
                    }
                }),
                (!cx.props.is_remote).then(|| rsx! {
                    if profile_picture.is_empty() {
                        rsx! (
                            div {
                                class: "pfp"
                            }  
                        )   
                        } else {
                            rsx!(
                                img {
                                    src: "{profile_picture}",
                                    height: "50",
                                    width: "50",
        
                                }
                            )
                        }
                }),
                p {
                    "{cx.props.message}",
                },
                (cx.props.is_remote).then(|| rsx! {
                    if profile_picture2.clone().is_empty() {
                        rsx! (
                            div {
                                class: "pfp"
                            }  
                        )   
                        } else {
                            rsx!(
                                img {
                                    src: "{profile_picture2}",
                                    height: "50",
                                    width: "50",
        
                                }
                            )
                        }
                }),
                (!cx.props.is_remote).then(|| rsx! {
                    span {
                        class: "box-drawing",
                        "🭾"
                    }
                })
            }
        }
    })
}
