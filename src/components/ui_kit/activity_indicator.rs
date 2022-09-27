use dioxus::prelude::*;
use warp::{crypto::{DID}, multipass::identity::IdentityStatus};

use crate::Account;

#[derive(PartialEq, Props)]
pub struct Props {
    inline: bool,
    remote_did: DID,
    account: Account,
}

#[allow(non_snake_case)]
pub fn ActivityIndicator(cx: Scope<Props>) -> Element {
    let identity = &cx.props.account;
    
    let status = match identity.read().identity_status(&cx.props.remote_did) {
        Ok(s) => s,
        Err(_) => IdentityStatus::Offline,
    };

    let main_class = match cx.props.inline {
        true => "inline",
        false => "icon-icon",
    };

    let bubble_class = match status {
        IdentityStatus::Online => "online",
        IdentityStatus::Offline => "offline",
    };

    cx.render(rsx! {
        div {
            class: "activity {main_class}",
            div { class: "bubble {bubble_class}" },
            p {
                "{status}"
            }
        }
    })
}
