use dioxus::prelude::*;
use utils::Account;

use crate::{components::main::friends::friend::Friend, Messaging};

use super::FriendListAlpha;

#[inline_props]
#[allow(non_snake_case)]
pub fn FriendsList(
    cx: Scope,
    account: Account,
    messaging: Messaging,
    add_error: UseState<String>,
    disp_friends: UseState<Vec<FriendListAlpha>>,
) -> Element {
    log::debug!("rendering FriendsList");

    let alpha: Vec<_> = "abcdefghijklmnopqrstuvwxyz"
        .to_uppercase()
        .chars()
        .collect();

    cx.render(rsx! {
        div {
            class: "main",
            div {
                class: "friends-list",
                disp_friends.iter().map(|friends_per_char_list| {
                    let first_username_char = friends_per_char_list.letter;
                    rsx!(
                    div {
                        class: "friends-separator",
                        h5 {
                            id: "{first_username_char}",
                            "{first_username_char}"
                        }
                    }
                    friends_per_char_list.friends.iter().map(|user| {
                            rsx!(
                                Friend {
                                    account: account.clone(),
                                    messaging: messaging.clone(),
                                    friend: user.did.clone(),
                                    friend_username: user.username.clone(),
                                    on_chat: move |_| {
                                        add_error.set("".into());
                                        use_router(&cx).push_route("/main", None, None);
                                    }
                                }
                            )
                        }),
                    )
                }),
            },
            ul {
                alpha.iter().map(|letter| {
                    rsx!( li { a { href: "#{letter}", prevent_default: "onclick", rel: "noopener noreferrer", "{letter}", } } )
                })
            }
        }
    })
}
