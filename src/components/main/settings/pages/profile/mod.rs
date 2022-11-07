use dioxus::prelude::*;

use crate::{
    components::ui_kit::{button::Button, icon_input::IconInput, photo_picker::PhotoPicker},
    Account, LANGUAGE,
};
use dioxus::events::FormEvent;
use dioxus_heroicons::outline::Shape;
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

    // todo: why does this not make the status persist when the settings page is reloaded/
    let status_msg = mp
        .read()
        .get_own_identity()
        .ok()
        .and_then(|i| i.status_message())
        .unwrap_or_else(String::new);

    let set_status = move |_: _| {
        let mp = mp.clone();
        edit.set(false);
        //TODO: Change to using `MultiPass::update_identity`
        let mut my_identity = match mp.write().get_own_identity() {
            Ok(me) => me,
            Err(_) => Identity::default(),
        };
        my_identity.set_status_message(Some((*status.current()).clone()));
        //println!("setting status: {}", &*status.current());
    };
    let set_status2 = set_status.clone();

    cx.render(rsx! {
        div {
            id: "page_profile",
            div {
                class: "profile_header",
                div {
                    class: "profile_picture",
                    PhotoPicker {
                        account: cx.props.account.clone(),
                    },
                }
            },
            div {
                div {
                    class: "status",
                    label {
                        "Status Message"
                    },
                },
                div {
                    class: "change-status",
                div {
                    class: "input_status",
                    IconInput {
                        icon: Shape::PencilAlt,
                        placeholder: status_msg,
                        on_change: move |e: FormEvent| {
                            status.set(e.value.clone());
                        },
                        on_enter: set_status
                    },
                },
                div {
                    Button {
                        text: l.save_status.to_string(),
                        icon: Shape::Check,
                        on_pressed: move |_| set_status2(()),
                    }
                }

            }
            },
        }
    })
}
