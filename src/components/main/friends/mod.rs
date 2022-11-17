pub mod friend;
pub mod request;
pub mod sidebar;

use std::{collections::HashSet, time::Duration};

use crate::{
    components::main::friends::{friend::Friend, sidebar::Sidebar},
    utils, Account, Messaging,
};

use dioxus::prelude::*;

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
    let friends = use_state(&cx, || HashSet::new());

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

                if *friends != friends_list && friends_list.len() > 0 {
                    log::debug!("updating friends list ");
                    let new_friends_list = order_friend_list(&friends_list, &mp);
                    friends_grouped_per_first_letter.set(new_friends_list);
                    friends.set(friends_list);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        },
    );

    let alpha: Vec<_> = "abcdefghijklmnopqrstuvwxyz"
        .to_uppercase()
        .chars()
        .collect();

    cx.render(rsx! {
        div {
            id: "friends",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "friends-list",
                    friends_grouped_per_first_letter.iter().map(|friends_per_char_list| {
                        let first_username_char = friends_per_char_list.first_letter_friends;
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
                alpha.iter().map(|letter| {
                    rsx!( li { a { href: "#{letter}", prevent_default: "onclick", rel: "noopener noreferrer", "{letter}", } } )
                })
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

    // Get all friends username
    for friend_did in friend_did_list.iter() {
        let _friend_username = utils::get_username_from_did(friend_did.clone(), account);
        let _friend_username_and_did = UsernameAndDID {
            username: _friend_username,
            did: friend_did.clone(),
        };
        username_did.push(_friend_username_and_did);
    }

    username_did.sort_by(|a, b| a.username.cmp(&b.username));

    // Get the first letter username that start the list
    let mut old_letter: char = username_did[0]
        .username
        .to_uppercase()
        .chars()
        .next()
        .unwrap();

    // Group friends per first username letter
    for (_friend, is_last_friend) in username_did
        .iter()
        .enumerate()
        .map(|(i, f)| (f, i == username_did.len() - 1))
    {
        let first_letter_friend_username = _friend.username.to_uppercase().chars().next().unwrap();

        if old_letter != first_letter_friend_username {
            sort_friends_and_add_on_friend_list_alpha(
                &mut group_of_friends_with_same_first_username_letter,
                &mut friends_grouped_per_first_letter,
                old_letter,
            );
            group_of_friends_with_same_first_username_letter.clear();
        }

        old_letter = first_letter_friend_username;
        group_of_friends_with_same_first_username_letter.push(_friend.clone());

        if is_last_friend {
            sort_friends_and_add_on_friend_list_alpha(
                &mut group_of_friends_with_same_first_username_letter,
                &mut friends_grouped_per_first_letter,
                old_letter,
            );
        }
    }
    friends_grouped_per_first_letter
}

fn sort_friends_and_add_on_friend_list_alpha(
    group_of_friends_with_same_first_username_letter: &mut Vec<UsernameAndDID>,
    friends_grouped_per_first_letter: &mut Vec<FriendListAlpha>,
    old_letter: char,
) {
    group_of_friends_with_same_first_username_letter
        .sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));

    friends_grouped_per_first_letter.push(FriendListAlpha {
        first_letter_friends: old_letter,
        friends: group_of_friends_with_same_first_username_letter.clone(),
    });
}
