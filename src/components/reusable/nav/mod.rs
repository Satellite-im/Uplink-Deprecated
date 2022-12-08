use ::utils::notifications::PushNotification;
use dioxus::{prelude::*};
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use state::{Actions, STATE};
use ui_kit::{
    button::{self, Button},
    context_menu::{ContextItem, ContextMenu},
    numeric_indicator::NumericIndicator,
};

use crate::{Account, Messaging, LANGUAGE};
use warp::{
    multipass::MultiPassEventKind,
    raygun::{Conversation, ConversationType},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NavEvent {
    Home,
    Files,
    Friends,
    Profile,
    Settings,
}

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Nav(cx: Scope<Props>) -> Element {
    log::debug!("rendering reusable Nav");
    let state = use_atom_ref(&cx, STATE).clone();
    let l = use_atom_ref(&cx, LANGUAGE).read().clone();
    let rg = cx.props.messaging.clone();
    let multipass = cx.props.account.clone();
    let reqCount = use_state(&cx, || {
        multipass.list_incoming_request().unwrap_or_default().len()
    });

    let route = use_route(&cx).last_segment();

    let active = match route {
        Some(r) => match r {
            "main" => NavEvent::Home,
            "files" => NavEvent::Files,
            "friends" => NavEvent::Friends,
            "settings" => NavEvent::Settings,
            _ => NavEvent::Home,
        },
        None => todo!(),
    };

    use_future(
        &cx,
        (reqCount, &multipass, &rg),
        |(reqCount, mut multipass, mut rg)| async move {
            // Used to make sure everything is initialized before proceeding.
            let new_friend_request_notification = l.new_friend_request.to_string().to_owned();

            let mut stream = loop {
                match multipass.subscribe() {
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

            // one can process other events such as event sent, closed, accepted, etc
            // Note: should only use events related to `FriendRequest` here
            while let Some(event) = stream.next().await {
                match event {
                    MultiPassEventKind::FriendRequestReceived { from } => {
                        // Use to show the name or did of who its from
                        let name_or_did = multipass
                            .get_identity(from.clone().into())
                            .ok()
                            .and_then(|list| list.first().cloned())
                            .map(|id| id.username())
                            .unwrap_or_else(|| from.to_string());

                        PushNotification(
                            new_friend_request_notification.clone(),
                            // "New Friend Request".to_owned(),
                            format!("{} sent a friend request", name_or_did),
                            // "Come see who it is!".to_owned(),
                            ::utils::sounds::Sounds::FriendReq,
                        );
                        log::debug!("updating friend request count");
                        // Note, this will increase the counter. Maybe use a separate task to check the list or use other events to decrease it
                        reqCount.with_mut(|count| *count += 1);
                    }
                    MultiPassEventKind::IncomingFriendRequestRejected { .. } => {
                        log::debug!("friend request rejected");
                        if *(reqCount.get()) != 0 {
                            log::debug!("close friend request");
                            reqCount.with_mut(|count| *count -= 1);
                        }
                    }
                    MultiPassEventKind::IncomingFriendRequestClosed { .. } => {
                        log::debug!("friend reqeust cancelled");
                        if *(reqCount.get()) != 0 {
                            log::debug!("close friend request");
                            reqCount.with_mut(|count| *count -= 1);
                        }
                    }
                    MultiPassEventKind::FriendAdded { did } => {
                        log::debug!("added friend: {}", &did);
                        if *(reqCount.get()) != 0 {
                            reqCount.with_mut(|count| *count -= 1);
                        }
                        log::debug!("creating chat");
                        let _result = rg.create_conversation(&did).await;
                    }
                    MultiPassEventKind::FriendRemoved { did } => {
                        log::debug!("removing friend {}", &did);
                        if let Ok(convs) = rg.list_conversations().await {
                            let to_remove: Vec<&Conversation> = convs
                                .iter()
                                .filter(|c| c.conversation_type() == ConversationType::Direct)
                                .filter(|c| c.recipients().contains(&did))
                                .collect();
                            for c in to_remove {
                                match rg.delete(c.id(), None).await {
                                    Ok(_) => {
                                        state.write().dispatch(Actions::RemoveConversation(c.id()));
                                        log::info!("successfully deleted conversation")
                                    }
                                    Err(error) => {
                                        log::error!("error when deleting conversation: {error}")
                                    }
                                };
                            }
                        }
                    }
                    _ => {}
                }
            }
        },
    );

    cx.render(rsx! {
        div {
            class: "nav",
            Button {
                on_pressed: move |_| {
                    use_router(&cx).push_route("/main", None, None);
                },
                state: if active.eq(&NavEvent::Home) {
                    button::State::Primary
                } else {
                    button::State::Secondary
                }
                icon: Shape::ChatBubbleBottomCenterText
            },
            Button {
                on_pressed: move |_| {
                    use_router(&cx).push_route("/main/files", None, None);
                },
                state: if active.eq(&NavEvent::Files) {
                    button::State::Primary
                } else {
                    button::State::Secondary
                },
                icon: Shape::Folder
            },
            div {
                class: {
                    if active.eq(&NavEvent::Friends) {
                        format_args!("has_indicator parent_active")
                    } else {
                        format_args!("has_indicator")
                    }
                },
                Button {
                    on_pressed: move |_| {
                        use_router(&cx).push_route("/main/friends", None, None);
                    },
                    state: if active.eq(&NavEvent::Friends) {
                        button::State::Primary
                    } else {
                        button::State::Secondary
                    }
                    icon: Shape::Users
                },
                (*reqCount.get() > 0).then(|| rsx!(
                    NumericIndicator {
                        count: *reqCount.get()
                    }
                )),
            }
            span {
                id: "settings-cog",
                ContextMenu {
                    parent: String::from("settings-cog"),
                    devmode: true,
                    items: cx.render(rsx! {
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("General")
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Profile")
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Extensions")
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            text: String::from("Developer")
                        },
                        hr {},
                        ContextItem {
                            onpressed: move |_| {},
                            icon: Shape::FolderOpen,
                            text: String::from("Open Cache")
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            icon: Shape::CodeBracketSquare,
                            text: String::from("Toggle Developer")
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            icon: Shape::Beaker,
                            text: String::from("Toggle Extensions")
                        },
                        ContextItem {
                            onpressed: move |_| {},
                            icon: Shape::Trash,
                            text: String::from("Delete Account"),
                            danger: true
                        },
                    })
                },
                Button {
                    on_pressed: move |_| {
                        use_router(&cx).push_route("/main/settings", None, None);
                    },
                    state: if active.eq(&NavEvent::Settings) {
                        button::State::Primary
                    } else {
                        button::State::Secondary
                    },
                    icon: Shape::Cog
                },
            }
        }
    })
}
