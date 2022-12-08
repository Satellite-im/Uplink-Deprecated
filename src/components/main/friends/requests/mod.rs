use crate::components::main::friends::request::FriendRequest;
use crate::{Account, LANGUAGE};

use std::{collections::HashSet, time::Duration};

use dioxus::prelude::*;

#[inline_props]
#[allow(non_snake_case)]
pub fn FriendRequests(cx: Scope, account: Account, add_error: UseState<String>) -> Element {
    log::debug!("rendering FriendRequests");

    let l = use_atom_ref(&cx, LANGUAGE).read();
    let incomingRequestsLang = { l.incoming_requests.to_string() };
    let outgoingRequestsLang = { l.outgoing_requests.to_string() };

    let incoming: &UseState<HashSet<_>> = use_state(&cx, || {
        HashSet::from_iter(account.list_incoming_request().unwrap_or_default())
    });
    let outgoing: &UseState<HashSet<_>> = use_state(&cx, || {
        HashSet::from_iter(account.list_outgoing_request().unwrap_or_default())
    });

    use_future(
        &cx,
        (incoming, outgoing, &account.clone()),
        |(incoming, outgoing, account)| async move {
            loop {
                let incoming_list: HashSet<_> =
                    HashSet::from_iter(account.list_incoming_request().unwrap_or_default());
                let outgoing_list: HashSet<_> =
                    HashSet::from_iter(account.list_outgoing_request().unwrap_or_default());

                if *incoming != incoming_list {
                    log::debug!("updating incoming friend requests ");
                    incoming.set(incoming_list);
                }

                if *outgoing != outgoing_list {
                    log::debug!("updating outgoing friend requests ");
                    outgoing.set(outgoing_list);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        },
    );

    cx.render(rsx! {
        div {
            div {
                (!incoming.is_empty()).then(|| rsx!(
                    label {
                    "{incomingRequestsLang}"
                    },
                    div {
                        incoming.iter().map(|request| rsx!(
                            FriendRequest {
                                account: account.clone(),
                                request: request.clone(),
                                on_accept: move |_| {
                                    match account.clone()
                                        .accept_request(&request.from())
                                    {
                                        Ok(_) => {
                                            add_error.set("".into());
                                        },
                                        Err(_) => {
                                            // TODO: Catch this and display it
                                            println!("Error");
                                        },
                                    }
                                },
                                on_deny: move |_| {
                                    match account.clone()
                                        .deny_request(&request.from())
                                    {
                                        Ok(_) => {
                                            add_error.set("".into());
                                        },
                                        Err(_) => {
                                            // TODO: Catch this and display it
                                            println!("Error");
                                        },
                                    }
                                },
                                deny_only: false,
                            }
                        )),
                    }
                )),
                (!outgoing.is_empty()).then(|| rsx!(
                    label {
                        "{outgoingRequestsLang}"
                    },
                    div {
                        outgoing.iter().map(|request| rsx!(
                            FriendRequest {
                                account: account.clone(),
                                request: request.clone(),
                                on_deny:  move |_| {
                                    match account.clone()
                                        .close_request(&request.to())
                                    {
                                        Ok(_) => {
                                            add_error.set("".into());
                                        },
                                        Err(_) => {
                                            // TODO: Catch this and display it
                                            println!("Error");
                                        },
                                    }
                                },
                                on_accept: move |_| {},
                                deny_only: true,
                            }
                        )),
                    }
                )),
            }
        },
    })
}
