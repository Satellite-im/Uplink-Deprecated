use dioxus::prelude::*;

use crate::Account;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Profile(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "page_profile",
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Status Message"
                    },
                    p {
                        "Set a custom status message and let people know what's happening."
                    }
                },
                div {
                    class: "interactive",
                    "input here"
                }
            }
        }
    })
}
