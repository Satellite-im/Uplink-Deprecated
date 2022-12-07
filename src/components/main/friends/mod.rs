pub mod find;
pub mod list_type_button;
pub mod request;
pub mod requests;
pub mod sidebar;
pub mod users_list;

use crate::{
    components::main::friends::sidebar::Sidebar,
    components::reusable::nav::Nav,
    components::{
        main::friends::{
            find::FindFriends, list_type_button::ListTypeButton, requests::FriendRequests,
            users_list::UsersList,
        },
        reusable::page_header,
    },
    Account, Messaging, STATE,
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

    let state = use_atom_ref(&cx, STATE);
    let sidebar_visibility = match state.read().hide_sidebar {
        false => "mobile-sidebar-visible",
        true => "mobile-sidebar-hidden",
    };

    cx.render(rsx! {
        div {
            id: "friends",
            class: "mobile-sidebar-hidden",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "mobile-wrap",
                    FindFriends { account: cx.props.account.clone(), add_error: add_error.clone(), is_compact: true },
                },
                div{
                   class: "scroll-container",
                   div {
                    class: "mobile-wrap",
                        FriendRequests { account: cx.props.account.clone(), add_error: add_error.clone() },
                    }
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
                        class:"main",
                        UsersList{
                            account:&cx.props.account,
                            messaging: &cx.props.messaging,
                            show_friend_list:**show_friend_list,}

                    },
                },
                span {
                    class: "hidden-on-desktop mobile-nav",
                    Nav {
                        account: cx.props.account.clone(),
                    }
                }
            },
        }
    })
}
