use crate::{
    components::ui_kit::{badge::Badge, button::Button, icon_input::IconInput, popup::Popup},
    Account, LANGUAGE,
};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
<<<<<<< HEAD
use warp::multipass::identity::Identity;
=======
use warp::{crypto::DID, multipass::identity::Identity};
use crate::{
    components::ui_kit::{badge::Badge, button::Button, icon_input::IconInput, popup::Popup}, Account, LANGUAGE,
};
>>>>>>> 3d2b22b637db81dddc641787cd7779269b12ba2d

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

<<<<<<< HEAD
    let my_identity = mp.read().get_own_identity().unwrap();
        let l = use_atom_ref(&cx, LANGUAGE).read();
=======
    let my_identity = match mp.read().get_own_identity() {
        Ok(me) => me,
        Err(_) => Identity::default(),
    };
    let l = use_atom_ref(&cx, LANGUAGE).read();
>>>>>>> 3d2b22b637db81dddc641787cd7779269b12ba2d
    let badgesString = l.badges.to_string();
    let locationString = l.location.to_string();
    let friendString = l.friends.to_string();
    let aboutString = l.about.to_string();
    let noAboutString = l.no_about_message.to_string();
    let developementString = l.developement.to_string();

    let username = my_identity.username();
    let badges = my_identity.available_badges();
    let friends = use_state(&cx, Vec::new);
    friends.set(mp.read().list_friends().unwrap_or_default());

    let friend_count = friends.clone().len();

    let edit = use_state(&cx, || false);
    let status = use_state(&cx, || "".to_string());
    let disabled = status.len() == 0;

    let set_status = move |_: _| {
        let mp = mp.clone();
        if !disabled {
            edit.set(false);
            //TODO: Change to using `MultiPass::update_identity`
            let mut my_identity = match mp.write().get_own_identity() {
                Ok(me) => me,
                Err(_) => Identity::default(),
            };
            my_identity.set_status_message(Some(status.to_string()));
        }
    };

    let did = my_identity.did_key();

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
                            }
                        },
                        div {
                            class: "profile-body",
                            h3 {
                                class: "username",
                                "{username}"
                            },
                            if **edit {rsx! (
                                div {
                                    class: "change-status",
                                    IconInput {
                                        icon: Shape::PencilAlt,
                                        placeholder: l.status_placeholder.to_string(),
                                        value: status.to_string(),
                                        on_change: move |e: FormEvent| status.set(e.value.clone()),
                                        on_enter: set_status
                                    }
                                },
                                if disabled {rsx!(
                                    Button {
                                        text: l.save_status.to_string(),
                                        icon: Shape::Check,
                                        disabled: true,
                                        on_pressed: move |_| {},
                                    },
                                )} else {rsx!(
                                    Button {
                                        text: l.save_status.to_string(),
                                        icon: Shape::Check,
                                        on_pressed: move |_| {
                                            // TODO: Pending Voice & Video
                                            // set_status.call()
                                        }
                                    },
                                )}
                            )} else {rsx! (
                                p {
                                    class: "status",
                                    "{status}"
                                },
                                Button {
                                    text: l.edit_profile.to_string(),
                                    icon: Shape::PencilAlt,
                                    on_pressed: move |_| {
                                        edit.set(true);
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
                            hr {},
                            label {
                                "{developementString}",
                            },
                            p {
                                class: "small-did",
                                "{did}"
                            }
                        }
                    }
                )
            )
        },
    })
}
