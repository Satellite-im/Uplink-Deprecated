use dioxus::prelude::*;
use warp::{crypto::DID, multipass::identity::IdentityStatus, multipass::IdentityInformation};

use crate::Account;

#[derive(PartialEq, Props)]
pub struct Props {
    inline: bool,
    remote_did: DID,
    account: Account,
}

#[allow(non_snake_case)]
pub fn ActivityIndicator(cx: Scope<Props>) -> Element {
    log::debug!("rendering ActivityIndicator");
    let status = use_state(&cx, || IdentityStatus::Offline);

    let account = cx.props.account.clone();
    let remote_did = cx.props.remote_did.clone();

    use_future(&cx, (&account, status), |(account, status)| async move {
        loop {
            if let Ok(current_status) = account.identity_status(&remote_did) {
                if *status != current_status {
                    log::debug!("Updating activity indicator");
                    status.set(current_status);
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }
    });

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
