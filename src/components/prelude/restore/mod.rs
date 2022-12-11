use std::time::Duration;

use dioxus::desktop::use_window;
use dioxus::router::use_router;
use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::Icon;
use mime::*;
use regex::RegexSet;
use rfd::FileDialog;
use sir::css;
use tokio::time::sleep;
use ui_kit::{
    button::{self, Button},
    input::Input,
};
use warp::multipass::identity::IdentityUpdate;

use crate::{Account, LANGUAGE, WINDOW_SUFFIX_NAME};

// Remember: owned props must implement PartialEq!
#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Restore(cx: Scope<Props>) -> Element {
    log::debug!("rendering Auth");
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let router = use_router(&cx).clone();

    let passphrase = use_state(&cx, String::new);

    let mut mp = cx.props.account.clone();
    let mut new_account = move || {
        mp.get_sync_identity(&passphrase).unwrap();
        router.replace_route("/loading", None, None);
    };
    let mut new_account2 = new_account.clone();

    cx.render(rsx! {
        div {
            class: "auth",
            div {
                class: "container",
                rsx! {
                    h2 {
                        "{l.restore_account}",
                    },
                    div { class: "m-bottom" },
                    div {
                        class: "full-width",
                        Input {
                            icon: Shape::Identification,
                            value: passphrase.clone().to_string(),
                            placeholder: String::from("Insert a Seedphrase.."),
                            on_change: move | evt: FormEvent | {
                                passphrase.set(evt.value.clone());
                            },
                            on_enter: move |_| new_account(),
                        },
                        Button {
                            icon: Shape::Check,
                            text: String::from("Restore Account"),
                            on_pressed:  move |_| new_account2(),
                        },
                    }
                }
            }
        }
    })
}
