use dioxus::desktop::use_window;
use dioxus::router::use_router;
use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::Icon;
use sir::css;

use crate::{
    components::ui_kit::{
        button::{self, Button},
        icon_input::IconInput,
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
    log::debug!("rendering Auth");
    let window = use_window(&cx);
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let username = use_state(&cx, String::new);
    let valid_username = username.len() >= 4;
    let error = use_state(&cx, String::new);
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };

    let mp = cx.props.account.clone();
    let new_account = move || {
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
    let new_account2 = new_account.clone();

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


                    div {
                        class: "display",
                            rsx! {
                                Icon {
                                    icon: Shape::User,
                                    size: 30,
                                }
                            }
                    },


                    div { class: "m-bottom" },
                    div {
                        class: "full-width",
                        IconInput {
                            icon: Shape::Identification,
                            value: username.clone().to_string(),
                            placeholder: String::from("Choose a username.."),
                            on_change: move | evt: FormEvent | {
                                error.set(String::from(""));
                                if evt.value.len() > 26 {
                                    error.set(String::from("Maximum username length reached (26)"));
                                    return;
                                }
                                if evt.value.contains(char::is_whitespace) {
                                    error.set(String::from("Username cannot contain spaces."));
                                    return;
                                }

                                username.set(evt.value.clone());
                            },
                            on_enter: move |_| new_account(),
                        },
                        p {
                            class: "{error_class}",
                            "　{error}　"
                        },
                        Button {
                            icon: Shape::Check,
                            text: String::from("Create Account"),
                            disabled: !valid_username,
                            state: match valid_username {
                                true => button::State::Primary,
                                false => button::State::Secondary,
                            },
                            on_pressed:  move |_| new_account2(),
                        }
                    }
                }
            }
        }
    })
}
