
use dioxus::{events::FormEvent, prelude::*};
use dioxus::desktop::use_window;
use dioxus_heroicons::outline::Shape;
use dioxus::router::use_router;
use warp::{multipass::MultiPass, raygun::RayGun, sync::RwLock, tesseract::Tesseract};
use warp_mp_ipfs::config::MpIpfsConfig;
use warp_rg_ipfs::{config::RgIpfsConfig, Persistent};


use crate::{
    components::ui_kit::{
        button::{self, Button},
        icon_input::IconInput,
        loader::Loader,
        photo_picker::PhotoPicker,
    },
    LANGUAGE, WINDOW_SUFFIX_NAME, Account,
};

// Remember: owned props must implement PartialEq!
#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}


#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let window = use_window(&cx);
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let username = use_state(&cx, || String::from(""));
    let valid_username = username.len() >= 4;
    let error = use_state(&cx, || String::from(""));

    let multipass = cx.props.account.clone();

    let account_fetch_status = match multipass.read().get_own_identity() {
        Ok(i) => {
            window.set_title(&format!("{} - {}", i.username(), WINDOW_SUFFIX_NAME));
            use_router(&cx).push_route("/main", None, None);
            false
        }
        Err(_) => true,
    };
    let mp = multipass.clone();
    let new_account = move |_| match mp.write().create_identity(Some(username.as_str()), None) {
        Ok(_) => {
            window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
            use_router(&cx).push_route("/main", None, None);
        }
        Err(_) => error.set("".into()),
    };

    let mp2 = multipass.clone();
    let new_account_2 = move |_| match mp2.write().create_identity(Some(username.as_str()), None) {
        Ok(_) => {
            window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
            use_router(&cx).push_route("/main", None, None);
        }
        Err(_) => error.set("".into()),
    };

    cx.render(rsx! {
        div {
            class: "auth",
            div {
                class: "container",
                if account_fetch_status {
                    rsx! {
                        h2 {
                            "{l.create_account}",
                        },
                        label {
                            "{l.create_account_desc}",
                        },
                        div { class: "m-bottom" },
                        PhotoPicker {},
                        div { class: "m-bottom" },
                        div {
                            class: "full-width",
                            IconInput {
                                icon: Shape::Identification,
                                value: username.clone().to_string(),
                                placeholder: "Choose a username..".to_string(),
                                on_change: move | evt: FormEvent | {
                                    username.set(evt.value.clone());
                                },
                                on_enter: new_account_2,
                            },
                            div { class: "m-bottom-sm" },
                            Button {
                                icon: Shape::Check,
                                text: "Create Account".to_string(),
                                disabled: !valid_username,
                                state: match valid_username {
                                    true => button::State::Primary,
                                    false => button::State::Secondary,
                                },
                                on_pressed: new_account,
                            }
                        }
                    }
                } else {
                    rsx! {
                        Loader {
                            text: l.checking_account.clone()
                        }
                    }
                }
            }
        }
    })
}
