use dioxus::prelude::*;
use utils::Account;

use crate::{
    components::main::friends::{find::FindFriends, requests::FriendRequests},
    Messaging,
};

#[inline_props]
#[allow(non_snake_case)]
pub fn Sidebar(
    cx: Scope,
    account: Account,
    messaging: Messaging,
    add_error: UseState<String>,
) -> Element {
    log::debug!("rendering friends/Sidebar");

    cx.render(rsx!(
        crate::components::reusable::sidebar::Sidebar {
            account: account.clone(),
            messaging: messaging.clone(),
            FindFriends { account: account.clone(), add_error: add_error.clone(), is_compact: false },
            FriendRequests {account: account.clone(), add_error: add_error.clone()},
        }
    ))
}
