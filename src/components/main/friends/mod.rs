pub mod friend;
pub mod request;
pub mod sidebar;

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use crate::{
    components::main::friends::{friend::Friend, sidebar::Sidebar},
    Account, Messaging,
};

use crate::iutils::get_username_from_did;

use dioxus::prelude::*;
use dioxus_router::use_router;

#[derive(Clone, PartialEq)]
struct UsernameAndDID {
    username: String,
    did: warp::crypto::DID,
}

#[derive(PartialEq)]
struct FriendListAlpha {
    letter: char,
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
    let disp_friends = use_state(&cx, Vec::new);
    let friends = use_ref(&cx, HashSet::new);

    use_future(
        &cx,
        (friends, &cx.props.account.clone(), disp_friends),
        |(friends, mp, disp_friends)| async move {
            loop {
                let friends_list: HashSet<_> =
                    HashSet::from_iter(mp.read().list_friends().unwrap_or_default());

                if *friends.read() != friends_list {
                    log::debug!("updating friends list ");
                    if let Some(new_disp) = order_friend_list(&friends_list, &mp) {
                        *friends.write_silent() = friends_list;
                        disp_friends.set(new_disp);
                    }
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
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
                                let key = &user.did;
                                rsx!(
                                    div {
                                        key: "{key}",
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
) -> Option<Vec<FriendListAlpha>> {
    // Get all friends username. sort them
    let mut total_friends_list: Vec<UsernameAndDID> = friend_did_list
        .iter()
        .map(|did| {
            let _friend_username = get_username_from_did(did.clone(), account);
            UsernameAndDID {
                username: _friend_username,
                did: did.clone(),
            }
        })
        .collect();
    total_friends_list.sort_by(|a, b| a.username.cmp(&b.username));

    // split by letter. the vectors will be sorted already
    let mut sublists: HashMap<char, Vec<UsernameAndDID>> = HashMap::new();
    for friend in total_friends_list {
        let start = friend.username.to_uppercase().chars().next()?;
        match sublists.get_mut(&start) {
            Some(list) => list.push(friend),
            None => {
                let _ = sublists.insert(start, vec![friend]);
            }
        };
    }
    // turn into a vec and sort it.
    let mut ordered_sublists: Vec<FriendListAlpha> = sublists
        .iter()
        .map(|(k, v)| FriendListAlpha {
            letter: *k,
            friends: v.clone(),
        })
        .collect();

    ordered_sublists.sort_by(|l, r| l.letter.cmp(&r.letter));

    Some(ordered_sublists)
}
