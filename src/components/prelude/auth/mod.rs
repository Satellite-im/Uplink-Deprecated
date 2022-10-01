
use dioxus::{events::FormEvent, prelude::*};
use dioxus::desktop::use_window;
use dioxus_heroicons::outline::Shape;
use dioxus::router::use_router;
use sir::css;

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

fn remove_whitespace(s: &mut String) -> String {
    s.retain(|c| !c.is_whitespace());
    s.to_string()
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
            println!("valid username {}", valid_username);
            if valid_username {
                window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
                use_router(&cx).push_route("/main", None, None);
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
                                    if evt.value.len() < 26 {
                                        username.set(remove_whitespace(&mut evt.value.to_string()));
                                        if !error.is_empty() {
                                            error.set("".to_string());
                                        }
                                    } else {
                                        username.set(evt.value[..26].to_string());
                                        error.set("Maximum username length reached (26)".to_string());
                                    }
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
