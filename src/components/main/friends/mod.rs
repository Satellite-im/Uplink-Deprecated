pub mod friend;
pub mod request;
pub mod sidebar;

use crate::{
    components::main::friends::{friend::Friend, sidebar::Sidebar},
    utils, Account, Messaging,
};

use dioxus::prelude::*;
use warp::multipass::Friends;

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
    let friend_did_list = cx.props.account.list_friends().unwrap_or_default();

    let mut username_did_vector: Vec<UsernameAndDID> = Vec::new();
    for friend_did in friend_did_list.iter() {
        let _friend_username = utils::get_username_from_did(friend_did.clone(), &cx.props.account.clone());
        let _friend = UsernameAndDID {username: _friend_username, did: friend_did.clone()};
        username_did_vector.push(_friend);
    }
    username_did_vector.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));

    cx.render(rsx! {
        div {
            id: "friends",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "friends-list",
                    username_did_vector.iter().map(|user| rsx!(
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
