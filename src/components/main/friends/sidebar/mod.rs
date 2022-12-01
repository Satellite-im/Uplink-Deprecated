use crate::{
    components::main::friends::{find::FindFriends, requests::FriendRequests},
    Account,
};
use dioxus::prelude::*;

#[inline_props]
#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope, account: Account, add_error: UseState<String>) -> Element {
    log::debug!("rendering friends/Sidebar");

    cx.render(rsx!(
        crate::components::reusable::sidebar::Sidebar {
            account: cx.props.account.clone(),
            FindFriends { account: account.clone(), add_error: add_error.clone(), is_compact: false },
            FriendRequests {account: account.clone(), add_error: add_error.clone()},
        }
    ))
}
