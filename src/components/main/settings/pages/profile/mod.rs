use dioxus::prelude::*;

mod status_msg;
mod username;

use crate::Account;

use ui_kit::photo_picker::PhotoPicker;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[inline_props]
#[allow(non_snake_case)]
pub fn Profile(cx: Scope<Props>, account: Account) -> Element {
    log::debug!("rendering settings/pages/Profile");

    cx.render(rsx! {
        div {
            id: "page_profile",
            class: "padded",
            div {
                class: "profile-header",
                div {
                    class: "profile-picture",
                    PhotoPicker {
                        account: cx.props.account.clone(),
                    },
                }
            },
            div {
                username::Username{
                    account: account.clone(),
                },
                status_msg::StatusMsg{
                    account:account.clone(),
                }
            },
        }
    })
}
