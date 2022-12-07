use dioxus::prelude::*;
use utils::Account;
mod friend_list_tile;

use std::{collections::HashSet, time::Duration};

use crate::{
    components::main::friends::users_list::{
        friend_list::friend_list_tile::FriendListTile, get_alpha, order_friend_list,
    },
    Messaging,
};

#[derive(Props, PartialEq)]
pub struct FriendListProps {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn FriendList(cx: Scope<FriendListProps>) -> Element {
    let disp_friends = use_state(&cx, Vec::new);
    let friends = use_ref(&cx, HashSet::new);
    let alpha = get_alpha();

    use_future(
        &cx,
        (friends, &cx.props.account.clone(), disp_friends),
        |(friends, mp, disp_friends)| async move {
            loop {
                let friends_list: HashSet<_> =
                    HashSet::from_iter(mp.list_friends().unwrap_or_default());

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

    let friend_list = rsx!(
    div {
                    class: "users-list",
                      disp_friends.iter().map(|friends_per_char_list| {
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
                                        FriendListTile {
                                            account: cx.props.account.clone(),
                                            messaging: cx.props.messaging.clone(),
                                            friend: user.did.clone(),
                                            friend_username: user.username.clone(),
                                            on_chat: move |_| {
                                                use_router(&cx).push_route("/main", None, None);
                                            }
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

    cx.render(friend_list)
}
