use dioxus::prelude::*;
use warp::{crypto::DID, multipass::identity::IdentityStatus};

use crate::Account;

#[derive(PartialEq, Props)]
pub struct Props {
    inline: bool,
    remote_did: DID,
    account: Account,
}

#[allow(non_snake_case)]
pub fn ActivityIndicator(cx: Scope<Props>) -> Element {
    let status = use_state(&cx, || IdentityStatus::Offline);

    if let Ok(current_status) = cx
        .props
        .account
        .read()
        .identity_status(&cx.props.remote_did)
    {
        status.set(current_status);
    };

    let main_class = match cx.props.inline {
        true => "inline",
        false => "icon-icon",
    };

    cx.render(rsx! {
        div {
            class: "activity {main_class}",
            div { class: "bubble {status}" },
            p {
                "{status}"
            }
        }
    })
}
