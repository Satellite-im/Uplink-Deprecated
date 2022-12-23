pub mod find;
pub mod list_type_button;
pub mod request;
pub mod requests;
pub mod sidebar;
pub mod users_list;

use crate::{
    components::main::friends::sidebar::Sidebar,
    components::main::friends::{
        find::FindFriends, list_type_button::ListTypeButton, requests::FriendRequests,
        users_list::UsersList,
    },
    components::reusable::nav::Nav,
    Account, Messaging,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
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
    log::debug!("rendering Friends");
    let add_error = use_state(&cx, String::new);

    let show_friend_list = use_state(&cx, || true);

    let incoming_requests =
        warp::async_block_in_place_uncheck(cx.props.account.list_incoming_request())
            .unwrap_or_default()
            .is_empty();
    let outgoing_requests =
        warp::async_block_in_place_uncheck(cx.props.account.list_outgoing_request())
            .unwrap_or_default()
            .is_empty();

    cx.render(rsx! {
        div {
            id: "friends",
            class: "mobile-sidebar-hidden",
            Sidebar { account: cx.props.account.clone(), messaging: cx.props.messaging.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "mobile-wrap",
                    FindFriends { account: cx.props.account.clone(), add_error: add_error.clone(), is_compact: true },
                },
                div{
                    class: "user-category",
                    ListTypeButton{
                        text:String::from("All"),
                        active: **show_friend_list,
                        on_pressed:move |_|show_friend_list.set(true)
                    },
                    ListTypeButton{
                        text:String::from("Blocked"),
                        active: !**show_friend_list,
                        on_pressed: move |_|show_friend_list.set(false)
                    },
                },
                div{
                   class: "scroll-container",
                   (incoming_requests || outgoing_requests).then(|| {
                       rsx! {
                           div {
                               class: "mobile-wrap",
                                   FriendRequests { account: cx.props.account.clone(), add_error: add_error.clone() },
                           },
                       }
                   }),
                   div{
                       class:"main",
                       UsersList {
                           account:cx.props.account.clone(),
                           messaging: cx.props.messaging.clone(),
                           show_friend_list:**show_friend_list,
                       },
                   },
                },
                span {
                    class: "hidden-on-desktop mobile-nav",
                    Nav {
                        account: cx.props.account.clone(),
                        messaging: cx.props.messaging.clone(),
                    }
                }
            },
        }
    })
}
