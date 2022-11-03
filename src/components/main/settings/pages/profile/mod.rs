use dioxus::prelude::*;

use crate::{
    components::ui_kit::{ badge::Badge, button::Button, icon_input::IconInput, popup::Popup, photo_picker::PhotoPicker },
    Account, LANGUAGE
};
use dioxus_heroicons::outline::Shape;
use dioxus::{events::FormEvent};
use warp::multipass::identity::Identity;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}


#[allow(non_snake_case)]
pub fn Profile(cx: Scope<Props>) -> Element {

    let l = use_atom_ref(&cx, LANGUAGE).read();
    let edit = use_state(&cx, || false);
    let status = use_state(&cx, String::new);
    let mp = cx.props.account.clone();
    let set_status = move |_: _| {
        let mp = mp.clone();
            edit.set(false);
            //TODO: Change to using `MultiPass::update_identity`
            let mut my_identity = match mp.write().get_own_identity() {
                Ok(me) => me,
                Err(_) => Identity::default(),
            };
            my_identity.set_status_message(Some(status.to_string()));
    };
    cx.render(rsx! {
        div {
            id: "page_profile",
            div {
                class: "",
                div {
                    class: "status",
                    label {
                        "Status Message"
                    },
                },
                div {
                    class: "row change-status",
                div {
                    class: "row input_status",
                    IconInput {
                        icon: Shape::PencilAlt,
                        placeholder: l.status_placeholder.to_string(),
                        value: status.to_string(),
                        on_change: move |e: FormEvent| status.set(e.value.clone()),
                        on_enter: set_status
                    },
                },
                div {
                    class: "",
                    Button {
                        text: l.save_status.to_string(),
                        icon: Shape::Check,
                        on_pressed: move |_| {},
                    }
                }

            }
            },
            div {
                class: "item",
                div {
                    class: "profile_picture",
                    div {
                        label {
                            "Choose a profile picture."
                        }
                        div {
                            PhotoPicker {
                                account: cx.props.account.clone(),
                            },
                        }
                    },
                },
            }
        }
    })
}
