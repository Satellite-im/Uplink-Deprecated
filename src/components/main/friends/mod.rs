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
    let add_error = use_state(&cx, String::new);
    let friend_did_list = cx.props.account.list_friends().unwrap_or_default();


    let mut username_did: Vec<UsernameAndDID> = Vec::new();
    let mut group_of_friends_with_same_first_username_letter: Vec<UsernameAndDID> = Vec::new();
    let mut friends_grouped_per_first_letter: Vec<FriendListAlpha> = Vec::new();
    let mut old_letter: char = 'A';

    for friend_did in friend_did_list.iter() {
        let _friend_username = utils::get_username_from_did(friend_did.clone(), &cx.props.account.clone());
        let _friend_username_and_did = UsernameAndDID {username: _friend_username, did: friend_did.clone()};
        username_did.push(_friend_username_and_did);
    }
    username_did.sort_by(|a, b| a.username.cmp(&b.username));

    for _friend in username_did.iter() {
        let first_letter_friend_username =  _friend.username.to_uppercase().chars().next().unwrap();

        if old_letter == first_letter_friend_username {
            group_of_friends_with_same_first_username_letter.push(_friend.clone());
        } else if !group_of_friends_with_same_first_username_letter.is_empty() {
            group_of_friends_with_same_first_username_letter.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));
            friends_grouped_per_first_letter.push(FriendListAlpha { first_letter_friends: old_letter, friends: group_of_friends_with_same_first_username_letter.clone()});
            group_of_friends_with_same_first_username_letter = vec![];
            group_of_friends_with_same_first_username_letter.push(_friend.clone());
        }
        old_letter = first_letter_friend_username;
    }
    
    cx.render(       
        rsx! {
        div {
            id: "friends",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "friends-list",
                    friends_grouped_per_first_letter.iter().map(|friends_per_char_list| {
                        rsx!(
                            FriendListSeparatorPerChar {first_letter:  friends_per_char_list.first_letter_friends}, 
                            friends_per_char_list.friends.iter().map(|user| {
                                rsx!(
                                Friend {
                                    account: cx.props.account.clone(),
                                    messaging: cx.props.messaging.clone(),
                                    friend: user.did.clone(),
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
        }
    })
}

#[derive(PartialEq, Eq, Props)]
pub struct PropsFriendSeparator {
    first_letter: char,
}

#[allow(non_snake_case)]
pub fn FriendListSeparatorPerChar(cx: Scope<PropsFriendSeparator>) -> Element {
    let first_letter = cx.props.first_letter;

     cx.render(rsx! {
        div {
            class: "friends-separator",
            h5 {
                "{first_letter}"
            }
        }
    })
}
