use crate::components::main::friends::request::FriendRequest;
use crate::{Account, LANGUAGE};

use std::{collections::HashSet};

use dioxus::prelude::*;
use futures::StreamExt;
use warp::multipass::MultiPassEventKind;

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
        |(incoming, outgoing, mut account)| async move {
            let mut stream = loop {
                match account.subscribe() {
                    Ok(stream) => break stream,
                    Err(e) => match e {
                        //Note: Used as a precaution for future checks
                        warp::error::Error::MultiPassExtensionUnavailable => {
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        //TODO: log error
                        //Note: Shouldnt give any other error but if it does to probably file as a bug
                        _ => return,
                    },
                };
            };

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

            while let Some(event) = stream.next().await {
                match event {
                    //Since we cannot manually grab the request object
                    //we will update outgoing request with the whole list
                    //regarding both incoming and outgoing
                    MultiPassEventKind::FriendRequestSent { .. } => {
                        let outgoing_list: HashSet<_> =
                            HashSet::from_iter(account.list_outgoing_request().unwrap_or_default());
                        if *outgoing != outgoing_list {
                            log::debug!("updating outgoing friend requests ");
                            outgoing.set(outgoing_list);
                        }
                    }
                    MultiPassEventKind::FriendRequestReceived { .. } => {
                        let incoming_list: HashSet<_> =
                            HashSet::from_iter(account.list_incoming_request().unwrap_or_default());
                        if *incoming != incoming_list {
                            log::debug!("updating incoming friend requests ");
                            incoming.set(incoming_list);
                        }
                    }
                    // Here we will iterate over the hashset and find the DID in the from field
                    // Although we could simply update the list completely, its cheaper to remove it from the state
                    MultiPassEventKind::IncomingFriendRequestClosed { did }
                    | MultiPassEventKind::IncomingFriendRequestRejected { did } => {
                        let Some(request) = incoming.iter().find(|req| req.from().eq(&did)) else {
                            continue
                        };

                        incoming.make_mut().remove(request);
                    }
                    MultiPassEventKind::OutgoingFriendRequestClosed { did }
                    | MultiPassEventKind::OutgoingFriendRequestRejected { did } => {
                        let Some(request) = outgoing.iter().find(|req| req.to().eq(&did)) else {
                            continue
                        };

                        outgoing.make_mut().remove(request);
                    }
                    // This event is emitted regardless of the accepting of an incoming or outgoing request
                    // so we will update both list without attempting to iterate
                    // although iteration might be cheaper, its to reduce the complexity
                    MultiPassEventKind::FriendAdded { .. } => {
                        let outgoing_list: HashSet<_> =
                            HashSet::from_iter(account.list_outgoing_request().unwrap_or_default());
                        if *outgoing != outgoing_list {
                            log::debug!("updating outgoing friend requests ");
                            outgoing.set(outgoing_list);
                        }

                        let incoming_list: HashSet<_> =
                            HashSet::from_iter(account.list_incoming_request().unwrap_or_default());
                        if *incoming != incoming_list {
                            log::debug!("updating incoming friend requests ");
                            incoming.set(incoming_list);
                        }
                    }
                    _ => {}
                }
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
                                        Err(e) => {
                                            // TODO: Catch this and display it
                                            log::error!("error accepting friend request: {e}");
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
                                        Err(e) => {
                                            // TODO: Catch this and display it
                                            log::error!("error denying friend request: {e}");
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
                                        Err(e) => {
                                            // TODO: Catch this and display it
                                            log::error!("error canceling friend request: {e}");
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
