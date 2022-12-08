mod blocked_list;
mod friend_list;

use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use utils::Account;

use crate::iutils::get_username_from_did;
use crate::Messaging;

use self::blocked_list::BlockedList;
use self::friend_list::FriendList;

use super::UsernameAndDID;

#[derive(Props, PartialEq)]
pub struct UserListProps {
    account: Account,
    messaging: Messaging,
    //show blocked users list when it is false
    show_friend_list: bool,
}

#[allow(non_snake_case)]
pub fn UsersList(cx: Scope<UserListProps>) -> Element {
    let users_list = match cx.props.show_friend_list {
        true => rsx!(FriendList {
            account: cx.props.account.clone(),
            messaging: cx.props.messaging.clone(),
        }),
        //show blocked users list
        false => rsx!(BlockedList {
            account: cx.props.account.clone(),
        }),
    };

    cx.render(users_list)
}

// The followings are reusable part in FriendList and BlockedList
#[derive(PartialEq)]
pub struct FriendListAlpha {
    letter: char,
    friends: Vec<UsernameAndDID>,
}

pub fn order_friend_list(
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

pub fn get_alpha() -> Vec<char> {
    "abcdefghijklmnopqrstuvwxyz"
        .to_uppercase()
        .chars()
        .collect()
}
