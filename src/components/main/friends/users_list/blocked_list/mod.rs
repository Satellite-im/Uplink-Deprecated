use dioxus::prelude::*;
use utils::Account;
mod blocked_list_tile;

use std::{collections::HashSet, time::Duration};

use crate::components::main::friends::users_list::{
    blocked_list::blocked_list_tile::BlockedListTile, get_alpha, order_friend_list,
};

#[derive(Props, PartialEq)]
pub struct BlockedListProps {
    account: Account,
}

#[allow(non_snake_case)]
pub fn BlockedList(cx: Scope<BlockedListProps>) -> Element {
    let block_users = use_ref(&cx, HashSet::new);
    let disp_block_users = use_state(&cx, Vec::new);
    let alpha = get_alpha();
    // same way as friend list to get the block user list
    use_future(
        &cx,
        (block_users, &cx.props.account.clone(), disp_block_users),
        |(block_users, mp, disp_block_users)| async move {
            loop {
                let block_users_list: HashSet<_> =
                    HashSet::from_iter(mp.block_list().await.unwrap_or_default());

                if *block_users.read() != block_users_list {
                    log::debug!("updating block users list ");
                    if let Some(new_disp) = order_friend_list(&block_users_list, &mp) {
                        *block_users.write_silent() = block_users_list;
                        disp_block_users.set(new_disp);
                    }
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        },
    );

    let blocked_list = rsx!(
        div {
            class: "users-list",
            disp_block_users.iter().map(|friends_per_char_list| {
                let first_username_char = friends_per_char_list.letter;
                rsx!(
                    div {
                        class: "users-separator",
                        h5 {
                            id: "{first_username_char}",
                            "{first_username_char}"
                        }
                    }
                    friends_per_char_list.friends.iter().map(|user| {
                        rsx!(
                            BlockedListTile {
                                account: cx.props.account.clone(),
                                friend: user.did.clone(),
                                friend_username: user.username.clone(),
                            }
                        )
                    }),
                )
            }),
        },
        ul {
            class: "a-z-list",
            alpha.iter().map(|letter| {
                rsx!( li { a { href: "#{letter}", prevent_default: "onclick", rel: "noopener noreferrer", "{letter}", } } )
            })
        }

    );

    cx.render(blocked_list)
}
