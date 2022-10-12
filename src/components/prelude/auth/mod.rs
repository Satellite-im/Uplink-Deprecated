use dioxus::desktop::use_window;
use dioxus::router::use_router;
use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use sir::css;

use crate::{
    components::ui_kit::{
        button::{self, Button},
        icon_input::IconInput,
        photo_picker::PhotoPicker,
    },
    Account, LANGUAGE, WINDOW_SUFFIX_NAME,
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
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };

    let mp = cx.props.account.clone();
    let new_account = move |_| {
        let username = username.trim();
        if username.is_empty() {
            error.set("Username is required".into())
        } else {
            match mp.write().create_identity(Some(username), None) {
                Ok(_) => {
                    window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
                    use_router(&cx).push_route("/loading", None, None);
                }
                Err(warp::error::Error::InvalidLength { .. }) => {
                    error.set("Username length is invalid".into())
                }
                Err(_) => error.set("Unexpected error has occurred".into()),
            }
        }
    };

    let mp2 = cx.props.account.clone();
    let new_account_2 = move |_| {
        let username = username.trim();
        if username.is_empty() {
            error.set("Username is required".into())
        } else {
            match mp2.write().create_identity(Some(username), None) {
                Ok(_) => {
                    window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
                    use_router(&cx).push_route("/loading", None, None);
                }
                Err(warp::error::Error::InvalidLength { .. }) => {
                    error.set("Username length is invalid".into())
                }
                Err(_e) => {
                    error.set("Unexpected error has occurred".into())
                }
            }
        }
    };

    cx.render(rsx! {
        div {
            class: "auth",
            div {
                class: "container",
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
                                error.set("".to_string());
                                if evt.value.len() > 26 {
                                    error.set("Maximum username length reached (26)".to_string());
                                    return;
                                }
                                if evt.value.contains(char::is_whitespace) {
                                    error.set("Username cannot contain spaces.".to_string());
                                    return;
                                }

                                username.set(evt.value.clone());
                            },
                            on_enter: new_account_2,
                        },
                        p {
                            class: "{error_class}",
                            "　{error}　"
                        },
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
            }
        }
    })
}
