pub mod friend;
pub mod request;
pub mod sidebar;

use crate::{
    components::main::friends::{friend::Friend, sidebar::Sidebar},
    utils, Account, Messaging,
};

use dioxus::prelude::*;
use warp::multipass::Friends;

#[derive(Clone)]
struct UsernameAndDID {
    username: String,
    did: warp::crypto::DID,
}

struct FriendListAlpha {
    first_letter_friends: char,
    friends: Vec<UsernameAndDID>,
}

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope<Props>) -> Element {
    log::debug!("rendering Friends");
    let add_error = use_state(&cx, String::new);
    let friend_did_list = cx.props.account.list_friends().unwrap_or_default();

    let mut username_did: Vec<UsernameAndDID> = Vec::new();
    let mut group_of_friends_with_same_first_username_letter: Vec<UsernameAndDID> = Vec::new();
    let mut friends_grouped_per_first_letter: Vec<FriendListAlpha> = Vec::new();
    let mut old_letter: char = 'A';

    for friend_did in friend_did_list.iter() {
        let _friend_username =
            utils::get_username_from_did(friend_did.clone(), &cx.props.account.clone());
        let _friend_username_and_did = UsernameAndDID {
            username: _friend_username,
            did: friend_did.clone(),
        };
        username_did.push(_friend_username_and_did);
    }
    username_did.sort_by(|a, b| a.username.cmp(&b.username));

    for _friend in username_did.iter() {
        let first_letter_friend_username = _friend.username.to_uppercase().chars().next().unwrap();

        if old_letter == first_letter_friend_username {
            group_of_friends_with_same_first_username_letter.push(_friend.clone());
        } else if !group_of_friends_with_same_first_username_letter.is_empty() {
            group_of_friends_with_same_first_username_letter
                .sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));
            friends_grouped_per_first_letter.push(FriendListAlpha {
                first_letter_friends: old_letter,
                friends: group_of_friends_with_same_first_username_letter.clone(),
            });
            group_of_friends_with_same_first_username_letter = vec![];
            group_of_friends_with_same_first_username_letter.push(_friend.clone());
        }
        old_letter = first_letter_friend_username;
    }

    cx.render(rsx! {
        div {
            id: "friends",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "friends-list",
                    friends_grouped_per_first_letter.iter().map(|friends_per_char_list| {
                        rsx!(
                            div {
                                class: "friends-separator",
                                h5 {
                                    id: "{friends_per_char_list.first_letter_friends}",
                                    "{friends_per_char_list.first_letter_friends}"
                                }
                            }
                            friends_per_char_list.friends.iter().map(|user| {
                                rsx!(
                                Friend {
                                    account: cx.props.account.clone(),
                                    messaging: cx.props.messaging.clone(),
                                    friend: user.did.clone(),
                                    friend_username: user.username.clone(),
                                    on_chat: move |_| {
                                        add_error.set("".into());
                                        use_router(&cx).push_route("/main", None, None);
                                    }
                                }
                            )

                        }
                        ),
                    )
                    }),

               }
            }
            ul {
                    li { a { href: "#A", "A", } }
                    li {a { href: "#B", "B", } }
                    li {a { href: "#C", "C", } }
                    li {a { href: "#D", "D", } }
                    li {a { href: "#E", "E", } }
                    li {a { href: "#F", "F", } }
                    li {a { href: "#G", "G", } }
                    li {a { href: "#H", "H", } }
                    li {a { href: "#I", "I", } }
                    li {a { href: "#J", "J", } }
                    li {a { href: "#K", "K", } }
                    li {a { href: "#L", "L", } }
                    li {a { href: "#M", "M", } }
                    li {a { href: "#N", "N", } }
                    li { a { href: "#O", "O", } }
                    li { a { href: "#P", "P", } }
                    li { a { href: "#Q", "Q", } }
                    li { a { href: "#R", "R", } }
                    li { a { href: "#S", "S", } }
                    li { a { href: "#T", "T", } }
                    li { a { href: "#U", "U", } }
                    li { a { href: "#V", "V", } }
                    li { a { href: "#W", "W", } }
                    li { a { href: "#X", "X", } }
                    li { a { href: "#Y", "Y", } }
                    li { a { href: "#Z", "Z", } }
                }
            }
    })
}
