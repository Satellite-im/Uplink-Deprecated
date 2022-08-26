use std::sync::Arc;

use dioxus::{prelude::*, events::FormEvent};
use dioxus_heroicons::outline::Shape;
use sir::{global_css};
use warp::{tesseract::Tesseract, sync::RwLock, multipass::MultiPass};
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{components::ui_kit::{loader::Loader, input::Input, photo_picker::PhotoPicker, button::{Button, self}}, TESSERACT, MULTIPASS, LANGUAGE};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    has_account: bool,
}

struct TessHolder(Tesseract);

impl AsRef<Tesseract> for TessHolder {
    fn as_ref(&self) -> &Tesseract {
        &self.0
    }
}

impl PartialEq for TessHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_unlock() == other.0.is_unlock()
    }
}

#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let tess = use_atom_ref(&cx, TESSERACT);

    let username = use_state(&cx, || String::from(""));
    let valid_username = username.len() >= 4;

    let multipass = use_atom_ref(&cx, MULTIPASS);
    let tess = tess.read().clone();
    let mp = use_future(&cx, (&tess,), |(tess,)| async move {
        warp_mp_ipfs::ipfs_identity_persistent(
            MpIpfsConfig::production("./.cache"),
            tess,
            None,
        ).await.map(|mp| Arc::new(RwLock::new(Box::new(mp) as Box<dyn MultiPass>)))
    });

    let account_fetch_status = match mp.value() {
        Some(Ok(val)) => {
            multipass.set(Some(val.clone()));
            match val.read().get_own_identity() {
                Ok(_) => {
                    use_router(&cx).push_route("/chat", None, None);
                    false
                },
                Err(_) => {
                    true
                },
            }
        },
        Some(Err(_)) => {
            // TODO: Make an error page and reroute there
            false
        },
        None => {
            false
        },
    };

    global_css! {"
        .auth {
            display: flex;
            justify-content: center;
            align-items: center;
            text-align: center;
            height: 80%;

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
    "}

    cx.render(rsx!{
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
                                oninput: move | evt: FormEvent | {
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
                                onclick: move |_| {
                                    println!("Clicked {}", username);
                                    // TODO: Create account in tess 
                                },
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