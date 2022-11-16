pub mod friend;
pub mod request;
pub mod sidebar;

use std::{collections::HashSet, time::Duration};

use crate::{
    components::main::friends::{friend::Friend, sidebar::Sidebar},
    utils, Account, Messaging,
};

use dioxus::prelude::*;
use warp::multipass::Friends;

#[derive(Clone, PartialEq)]
struct UsernameAndDID {
    username: String,
    did: warp::crypto::DID,
}

#[derive(PartialEq)]
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
    let friends_grouped_per_first_letter = use_state(&cx, || Vec::new());
    let friends = use_state(&cx, || {
        HashSet::from_iter(cx.props.account.list_friends().unwrap_or_default())
    });

    // First odered friends list
    let new_friends_list = order_friend_list(&friends, &cx.props.account.clone());
    friends_grouped_per_first_letter.set(new_friends_list);

    use_future(
        &cx,
        (
            friends,
            &cx.props.account.clone(),
            friends_grouped_per_first_letter,
        ),
        |(friends, mp, friends_grouped_per_first_letter)| async move {
            loop {
                let friends_list: HashSet<_> =
                    HashSet::from_iter(mp.read().list_friends().unwrap_or_default());

                if *friends != friends_list {
                    log::debug!("updating friends list ");
                    let new_friends_list = order_friend_list(&friends_list, &mp);
                    friends_grouped_per_first_letter.set(new_friends_list);
                    friends.set(friends_list);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        },
    );

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
                li { a { href: "#A", prevent_default: "onclick", rel: "noopener noreferrer", "A", } }
                li { a { href: "#B", prevent_default: "onclick", rel: "noopener noreferrer", "B", } }
                li { a { href: "#C", prevent_default: "onclick", rel: "noopener noreferrer", "C", } }
                li { a { href: "#D", prevent_default: "onclick", rel: "noopener noreferrer", "D", } }
                li { a { href: "#E", prevent_default: "onclick", rel: "noopener noreferrer", "E", } }
                li { a { href: "#F", prevent_default: "onclick", rel: "noopener noreferrer", "F", } }
                li { a { href: "#G", prevent_default: "onclick", rel: "noopener noreferrer", "G", } }
                li { a { href: "#H", prevent_default: "onclick", rel: "noopener noreferrer", "H", } }
                li { a { href: "#I", prevent_default: "onclick", rel: "noopener noreferrer", "I", } }
                li { a { href: "#J", prevent_default: "onclick", rel: "noopener noreferrer", "J", } }
                li { a { href: "#K", prevent_default: "onclick", rel: "noopener noreferrer", "K", } }
                li { a { href: "#L", prevent_default: "onclick", rel: "noopener noreferrer", "L", } }
                li { a { href: "#M", prevent_default: "onclick", rel: "noopener noreferrer", "M", } }
                li { a { href: "#N", prevent_default: "onclick", rel: "noopener noreferrer", "N", } }
                li { a { href: "#O", prevent_default: "onclick", rel: "noopener noreferrer", "O", } }
                li { a { href: "#P", prevent_default: "onclick", rel: "noopener noreferrer", "P", } }
                li { a { href: "#Q", prevent_default: "onclick", rel: "noopener noreferrer", "Q", } }
                li { a { href: "#R", prevent_default: "onclick", rel: "noopener noreferrer", "R", } }
                li { a { href: "#S", prevent_default: "onclick", rel: "noopener noreferrer", "S", } }
                li { a { href: "#T", prevent_default: "onclick", rel: "noopener noreferrer", "T", } }
                li { a { href: "#U", prevent_default: "onclick", rel: "noopener noreferrer", "U", } }
                li { a { href: "#V", prevent_default: "onclick", rel: "noopener noreferrer", "V", } }
                li { a { href: "#W", prevent_default: "onclick", rel: "noopener noreferrer", "W", } }
                li { a { href: "#X", prevent_default: "onclick", rel: "noopener noreferrer", "X", } }
                li { a { href: "#Y", prevent_default: "onclick", rel: "noopener noreferrer", "Y", } }
                li { a { href: "#Z", prevent_default: "onclick", rel: "noopener noreferrer", "Z", } }
                }
            }
    })
}

fn order_friend_list(
    friend_did_list: &HashSet<warp::crypto::DID>,
    account: &Account,
) -> Vec<FriendListAlpha> {
    let mut username_did: Vec<UsernameAndDID> = Vec::new();
    let mut group_of_friends_with_same_first_username_letter: Vec<UsernameAndDID> = Vec::new();
    let mut friends_grouped_per_first_letter: Vec<FriendListAlpha> = Vec::new();
    let mut old_letter: char = 'A';

    for friend_did in friend_did_list.iter() {
        let _friend_username = utils::get_username_from_did(friend_did.clone(), account);
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
    friends_grouped_per_first_letter
}
