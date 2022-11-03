use crate::{
    components::ui_kit::{badge::Badge, profile_picture::PFP, button::Button, icon_input::IconInput, popup::Popup, photo_picker::PhotoPicker},
    Account, LANGUAGE, utils,
};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use warp::multipass::identity::Identity;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    show: bool,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Profile<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
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
    let friends = use_state(&cx, || mp.read().list_friends().unwrap_or_default());
    let friend_count = use_state(&cx, || friends.clone().len());
    //let identity = mp.read().get_own_identity().unwrap();
    //identity.set_graphics(identity.graphics().)

    use_future(
        &cx,
        (friends, &mp, friend_count),
        |(friends, mp, friend_count)| async move {
            loop {
                let list = mp.read().list_friends().unwrap_or_default();
                if *friends != list {
                    friend_count.set(list.len());
                    friends.set(list);
                }
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
        },
    );

    let edit = use_state(&cx, || false);
    let status = use_state(&cx, String::new);
    let disabled = status.is_empty();
    let profile_picture = utils::get_pfp_from_did(my_identity.did_key(), &cx.props.account.clone());

    // let set_status = move |_: _| {
    //     let mp = mp.clone();
    //     if !disabled {
    //         edit.set(false);
    //         //TODO: Change to using `MultiPass::update_identity`
    //         let mut my_identity = match mp.write().get_own_identity() {
    //             Ok(me) => me,
    //             Err(_) => Identity::default(),
    //         };
    //         my_identity.set_status_message(Some(status.to_string()));
    //     }
    // };

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
                            div {
                                class: "profile-photo",
                                img {
                                    src: "{profile_picture}",
                                    height: "100",
                                    width: "100",
                                },
                            }
                        },
                        div {
                            class: "profile-body",
                            h3 {
                                class: "username",
                                "{username}"
                            }, {rsx! (
                                p {
                                    class: "status",
                                    "{status}"
                                },
                                Button {
                                    text: l.edit_profile.to_string(),
                                    icon: Shape::PencilAlt,
                                    on_pressed: move |_| {
                                        use_router(&cx).push_route("/main/settings", None, None);
                                    },
                                },
                            )}
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
