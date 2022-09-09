use std::sync::Arc;

use dioxus::{events::FormEvent, prelude::*};
use dioxus_desktop::use_window;
use dioxus_heroicons::outline::Shape;
use fermi::prelude::*;
use dioxus_router::use_router;
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
    DEFAULT_PATH, LANGUAGE, MULTIPASS, RAYGUN, WINDOW_SUFFIX_NAME,
};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let window = use_window(&cx);
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let username = use_state(&cx, || String::from(""));
    let valid_username = username.len() >= 4;
    let error = use_state(&cx, || String::from(""));

    let multipass = use_atom_ref(&cx, MULTIPASS);
    let raygun = use_atom_ref(&cx, RAYGUN);
    let tess = cx.props.tesseract.clone();

    let mp = use_future(&cx, (&tess,), |(tess,)| async move {
        warp_mp_ipfs::ipfs_identity_persistent(
            MpIpfsConfig::production(DEFAULT_PATH.read().clone()),
            tess,
            None,
        )
        .await
        .map(|mp| Arc::new(RwLock::new(Box::new(mp) as Box<dyn MultiPass>)))
    });

    let account_fetch_status = match mp.value() {
        Some(Ok(val)) => {
            *multipass.write() = Some(val.clone());

            match val.read().get_own_identity() {
                Ok(i) => {
                    let mp = val.clone();
                    let rg = use_future(&cx, (), |()| async move {
                        warp_rg_ipfs::IpfsMessaging::<Persistent>::new(
                            Some(RgIpfsConfig::production(DEFAULT_PATH.read().clone())),
                            mp,
                            None,
                        )
                        .await
                        .map(|rg| Arc::new(RwLock::new(Box::new(rg) as Box<dyn RayGun>)))
                    });
                    match rg.value() {
                        Some(Ok(rg)) => {
                            *raygun.write() = Some(rg.clone());
                            window.set_title(&format!("{} - {}", i.username(), WINDOW_SUFFIX_NAME));
                            use_router(&cx).push_route("/main", None, None);
                            false
                        }
                        Some(Err(_)) => {
                            //Note: Maybe want to return an error?
                            true
                        }
                        None => true,
                    }
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

    let new_account = move |_| match multipass
        .read()
        .clone()
        .unwrap()
        .write()
        .create_identity(Some(username.as_str()), None)
    {
        Ok(_) => {
            let mp = multipass.read().clone().unwrap().clone();
            let rg = use_future(&cx, (), |()| async move {
                warp_rg_ipfs::IpfsMessaging::<Persistent>::new(
                    Some(RgIpfsConfig::production(DEFAULT_PATH.read().clone())),
                    mp,
                    None,
                )
                .await
                .map(|rg| Arc::new(RwLock::new(Box::new(rg) as Box<dyn RayGun>)))
            });

            match rg.value() {
                Some(Ok(rg)) => {
                    *raygun.write() = Some(rg.clone());
                    window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
                    use_router(&cx).push_route("/main", None, None);
                }
                Some(Err(_)) => {
                    //Note: Maybe want to return an error?
                    error.set("".into())
                }
                None => error.set("".into()),
            }
        }
        Err(_) => error.set("".into()),
    };

    let new_account_2 = move |_| match multipass
        .read()
        .clone()
        .unwrap()
        .write()
        .create_identity(Some(username.as_str()), None)
    {
        Ok(_) => {
            let mp = multipass.read().clone().unwrap().clone();
            let rg = use_future(&cx, (), |()| async move {
                warp_rg_ipfs::IpfsMessaging::<Persistent>::new(
                    Some(RgIpfsConfig::production(DEFAULT_PATH.read().clone())),
                    mp,
                    None,
                )
                .await
                .map(|rg| Arc::new(RwLock::new(Box::new(rg) as Box<dyn RayGun>)))
            });

            match rg.value() {
                Some(Ok(rg)) => {
                    *raygun.write() = Some(rg.clone());
                    window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
                    use_router(&cx).push_route("/main", None, None);
                }
                Some(Err(_)) => {
                    //Note: Maybe want to return an error?
                    error.set("".into())
                }
                None => error.set("".into()),
            }
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
