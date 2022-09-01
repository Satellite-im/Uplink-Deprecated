use dioxus::{prelude::*, events::{FormData, FormEvent, MouseData}, core::UiEvent};
use dioxus_heroicons::outline::Shape;
use sir::global_css;
use warp::{multipass::identity::{Identity}, crypto::DID};

use crate::{
    components::ui_kit::{popup::Popup, button::Button, badge::Badge, icon_input::IconInput}, STATE, MULTIPASS,
};

#[derive(Props)]
pub struct Props<'a> {
    show: bool,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Profile<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    global_css! {"
        .profile {
            display: inline-flex;
            flex-direction: column;
            width: 100%;
            
            .background {
                height: 140px;
                width: 100%;
                background: var(--theme-text-muted);
                margin-bottom: 40px;
                position: relative;
                border-radius: 8px 8px 0 0;

                .profile-photo {
                    width: 80px;
                    height: 80px;
                    position: absolute;
                    bottom: -41px;
                    left: calc(50% - 41px);
                    border-radius: 80px;
                    background: var(--theme-text-muted);
                    border: 2px solid var(--theme-foreground);
                }
            }

            .profile-body {
                width: 100%;
                .status {
                    margin-bottom: 1rem;
                    font-size: 10pt;
                    color: var(--theme-text-muted);
                }
                .change-status {
                    padding: 1rem;
                }
                .button {
                    margin-bottom: 1rem;
                }
                .meta {
                    width: 100%;
                    display: inline-flex;
                    flex-direction: row;
                    justify-content: space-between;
                    font-size: 12px;

                    .badges, .location, .friend-count {
                        width: 33.33%;
                    }
                }
                .about {
                    p {
                        font-size: 10pt;
                        color: var(--theme-text-muted);
                    }
                }
            }
        }
    "};

    // Load Multipass & Raygun's Atom Ref
    let multipass = use_atom_ref(&cx, MULTIPASS);

    // Read their values from locks
    let mp = multipass.read().clone().unwrap().clone();

    let my_identity = match mp.read().get_own_identity() {
        Ok(me) => me,
        Err(_) => Identity::default(),
    };

    let username = my_identity.username();
    let status = match my_identity.status_message() {
        Some(status) => status,
        None => "No custom status set.".to_string(),
    };
    let badges = my_identity.available_badges();
    let friends = use_state(&cx, Vec::new);
    friends.set(match mp.read().list_friends() {
        Ok(f) => f
            .iter()
            .map(|friend| {
                match multipass
                    .read()
                    .clone()
                    .unwrap()
                    .read()
                    .get_identity(friend.clone().into())
                {
                    Ok(idents) => idents
                        .first()
                        .map(|i| i.did_key())
                        .unwrap_or_else(|| DID::default()),
                    Err(_) => DID::default(),
                }
            })
            .collect::<Vec<_>>(),
        Err(_) => vec![],
    });

    let friend_count = friends.clone().len();

    let edit = use_state(&cx, || false);
    let status = use_state(&cx, || "".to_string());
    let disabled = status.len() <= 0;

    let set_status = move |_:_| {
        let mp = mp.clone();
        if !disabled {
            edit.set(false);
            let mut my_identity = match mp.write().get_own_identity() {
                Ok(me) => me,
                Err(_) => Identity::default(),
            };
            my_identity.set_status_message(Some(status.to_string()));
        }
    };
    
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
                                        placeholder: "Some status message..".to_string(),
                                        value: status.to_string(),
                                        on_change: move |e: FormEvent| status.set(e.value.clone()),
                                        on_enter: set_status
                                    }
                                },
                                if disabled {rsx!(
                                    Button {
                                        text: "Save Status".to_string(),
                                        icon: Shape::Check,
                                        disabled: true,
                                        on_pressed: move |_| {},
                                    },
                                )} else {rsx!(
                                    Button {
                                        text: "Save Status".to_string(),
                                        icon: Shape::Check,
                                        on_pressed: move |_| {
                                            // TODO: Make this work
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
                                    text: "Edit Profile".to_string(),
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
                                        "Badges"
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
                                        "Location"
                                    },
                                    p {
                                        "Somewhere, USA"
                                    }
                                },
                                div {
                                    class: "friend-count",
                                    label {
                                        "Friends"
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
                                    "About"
                                },
                                p {
                                    "No about message set yet...",
                                }
                            }
                        }
                    }
                )
            )
        },
    })
}
