use dioxus::prelude::*;

pub mod sidebar;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
}

#[allow(non_snake_case)]
pub fn Contacts(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "contacts",
            sidebar::Sidebar { _account: cx.props.account.clone() },
            div {
                id: "content",
                label {
                    "{yourFriendsLang}"
                },
                div {
                    friends.iter().map(|user| rsx!(
                        Friend {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                            friend: user.clone(),
                            on_chat: move |_| {
                                add_error.set("".into());
                                cx.props.on_hide.call(());
                            }
                        }
                    )),
                }
            },
        }
    })
}
