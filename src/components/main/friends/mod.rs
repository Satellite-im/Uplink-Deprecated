pub mod friend;
pub mod request;
pub mod sidebar;

use crate::{
    components::main::friends::{friend::Friend, sidebar::Sidebar},
    utils, Account, Messaging,
};

use dioxus::prelude::*;
use std::{collections::HashSet, time::Duration};
use warp::multipass::Friends;

#[derive(PartialEq)]
struct UsernameAndDID {
    username: String, 
    did: warp::crypto::DID,
}

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope<Props>) -> Element {
    let add_error = use_state(&cx, String::new);
    let friends = use_state(&cx, || {
        Vec::new()
    });
     let friends_did = use_state(&cx, || {
        HashSet::from_iter(cx.props.account.list_friends().unwrap_or_default())
    });

    use_future(
        &cx,
        (friends, &cx.props.account.clone(), friends_did),
        |(friends, mp, friends_did)| async move {
            
            loop {
                let friends_list: HashSet<_> =
                HashSet::from_iter(mp.read().list_friends().unwrap_or_default());

                if *friends_did != friends_list {
                    let mut username_did_vector: Vec<UsernameAndDID> = Vec::new();
                    for friend_did in friends_list {
                        let _friend_username = utils::get_username_from_did(friend_did.clone(), &mp);
                        let _friend = UsernameAndDID {username: _friend_username, did: friend_did.clone()};
                        username_did_vector.push(_friend);
                        tokio::time::sleep(Duration::from_millis(300)).await;
                    }
                    username_did_vector.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));
                    friends.set(username_did_vector);
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
                    friends.iter().map(|user| rsx!(
                        Friend {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                            friend: user.did.clone(),
                            on_chat: move |_| {
                                add_error.set("".into());
                                use_router(&cx).push_route("/main", None, None);
                            }
                        }
                    )),
                }
            }
        }
    })
}
