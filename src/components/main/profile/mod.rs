use crate::{utils_internal, Account, LANGUAGE};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use std::collections::HashSet;
use ui_kit::{badge::Badge, button::Button, popup::Popup};
use warp::crypto::DID;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    show: bool,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Profile<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering Profile");
    // Read their values from locks
    let mp = cx.props.account.clone();

    let my_identity = mp.read().get_own_identity().unwrap();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let badgesString = l.badges.to_string();
    let locationString = l.location.to_string();
    let friendString = l.friends.to_string();
    let aboutString = l.about.to_string();
    let noAboutString = l.no_about_message.to_string();

    let username = my_identity.username();
    let badges = my_identity.available_badges();
    let friends: &UseState<HashSet<DID>> = use_state(&cx, || {
        HashSet::from_iter(mp.read().list_friends().unwrap_or_default().iter().cloned())
    });
    let friend_count = friends.current().len();
    //let identity = mp.read().get_own_identity().unwrap();
    //identity.set_graphics(identity.graphics().)

    use_future(&cx, (friends, &mp), |(friends, mp)| async move {
        loop {
            let list =
                HashSet::from_iter(mp.read().list_friends().unwrap_or_default().iter().cloned());
            if *friends.current() != list {
                log::debug!("modifying friends list");
                friends.set(list);
            }
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }
    });
    let status = my_identity.status_message().unwrap_or_default();
    let profile_picture =
        utils_internal::get_pfp_from_did(my_identity.did_key(), &cx.props.account.clone())
            .unwrap_or_default();

    cx.render(rsx! {
        Popup {
            on_dismiss: |_| cx.props.on_hide.call(()),
            hidden: !cx.props.show,
            children: cx.render(
                rsx!(
                    div {
                        class: "profile",
                        div {
                            class: "background",
                            if !profile_picture.is_empty() {
                                rsx!(
                                    img {
                                        class: "profile-photo",
                                        src: "{profile_picture}",
                                        height: "100",
                                        width: "100",
                                    }
                                )
                            } else {
                                rsx!(
                                    div {
                                        class: "profile-photo",
                                        rsx! {
                                            Icon {
                                                size: 40,
                                                icon: Shape::User,
                                            },
                                        }
                                    }
                                )
                            }
                        },
                        div {
                            class: "profile-body",
                            h3 {
                                class: "username",
                                "{username}"
                            },
                            p {
                                class: "status",
                                "{status}, here is status"
                            },
                            Button {
                                text: l.edit_profile.to_string(),
                                icon: Shape::PencilAlt,
                                on_pressed: move |_| {
                                    use_router(&cx).push_route("/main/settings/profile", None, None);
                                },
                            },
                            div {
                                class: "meta",
                                div {
                                    class: "badges",
                                    label {
                                        "{badgesString}"
                                    },
                                    div {
                                        class: "container",
                                        badges.iter().map(|_badge| rsx!(
                                            Badge {},
                                        ))
                                    }
                                },
                                div {
                                    class: "location",
                                    label {
                                        "{locationString}"
                                    },
                                    p {
                                        "Unknown"
                                    }
                                },
                                div {
                                    class: "friend-count",
                                    label {
                                        "{friendString}"
                                    }
                                    p {
                                        "{friend_count}"
                                    }
                                }
                            },
                            hr {},
                            div {
                                class: "about",
                                label {
                                    "{aboutString}"
                                },
                                p {
                                    "{noAboutString}",
                                }
                            },
                        }
                    }
                )
            )
        },
    })
}
