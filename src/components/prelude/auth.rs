use std::sync::Arc;

use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use sir::global_css;
use warp::{multipass::MultiPass, sync::RwLock, tesseract::Tesseract};
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{
    components::ui_kit::{
        button::{self, Button},
        input::Input,
        loader::Loader,
        photo_picker::PhotoPicker,
    },
    DEFAULT_PATH, LANGUAGE, MULTIPASS, RAYGUN,
};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let username = use_state(&cx, || String::from(""));
    let valid_username = username.len() >= 4;
    let error = use_state(&cx, || String::from(""));

    let multipass = use_atom_ref(&cx, MULTIPASS);
    let _raygun = use_atom_ref(&cx, RAYGUN);
    let tess = cx.props.tesseract.clone();
    let dp = DEFAULT_PATH.read().clone();

    let mp = use_future(&cx, (&tess,), |(tess,)| async move {
        warp_mp_ipfs::ipfs_identity_persistent(MpIpfsConfig::production(dp), tess, None)
            .await
            .map(|mp| Arc::new(RwLock::new(Box::new(mp) as Box<dyn MultiPass>)))
    });

    let account_fetch_status = match mp.value() {
        Some(Ok(val)) => {
            *multipass.write() = Some(val.clone());

            match val.read().get_own_identity() {
                Ok(_) => {
                    use_router(&cx).push_route("/main", None, None);
                    false
                }
                Err(_) => true,
            }
        }
        Some(Err(_)) => {
            // TODO: Make an error page and reroute there
            false
        }
        None => false,
    };

    // Start UI
    global_css! ("
        .auth {
            display: flex;
            justify-content: center;
            align-items: center;
            text-align: center;
            height: 80%;
            width: 100%;

            .container {
                min-width: 300px;
                max-width: 720px;
                position: relative;
                display: flex;
                align-items: center;
                justify-content: center;
                flex-direction: column;

                .full-width {
                    .input {
                        width: 100%;
                    }
                    .button {
                        width: 100%;
                    }
                }
            }
        }
    ");

    let new_account = move |_| match multipass
        .read()
        .clone()
        .unwrap()
        .write()
        .create_identity(Some(username), None)
    {
        Ok(_) => {
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
                            Input {
                                placeholder: "Choose a username..".to_string(),
                                on_change: move | evt: FormEvent | {
                                    username.set(evt.value.clone());
                                },
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
