use dioxus::desktop::use_window;
use dioxus::router::use_router;
use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::Icon;
use mime::*;
use regex::RegexSet;
use rfd::FileDialog;
use sir::css;
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
pub fn Auth(cx: Scope<Props>) -> Element {
    log::debug!("rendering Auth");
    let window = use_window(&cx);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let router = use_router(&cx).clone();
    let router_acc = router.clone();

    let username = use_state(&cx, String::new);
    let valid_username = username.len() >= 4;
    let error = use_state(&cx, String::new);
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };

    let profile_picture_state = use_state(&cx, String::new);

    let profile_picture_is_empty = profile_picture_state.is_empty();

    let mut mp = cx.props.account.clone();
    let mut new_account = move || {
        let username = username.trim();
        if username.is_empty() {
            error.set("Username is required".into())
        } else if username.len() < 4 || username.len() > 32 {
            error.set("Username needs to be between 4 and 32 characters long".into())
        } else {
            let username_regex_set =
                RegexSet::new([r"@", r"[[:^alnum:]&&[:^punct:]&&[^ ]]"]).unwrap();
            let matches = username_regex_set.matches(username);
            if matches.matched(0) {
                error.set("@ is not allowed in username".into())
            } else if matches.matched(1) {
                error.set("Illegal input in username".into())
            } else {
                match mp.create_identity(Some(username), None) {
                    Ok(_) => {
                        if !profile_picture_is_empty {
                            if let Err(e) =
                                mp.update_identity(IdentityUpdate::set_graphics_picture(
                                    profile_picture_state.to_string(),
                                ))
                            {
                                println!("{}", e);
                            }
                        }
                        window.set_title(&format!("{} - {}", username, WINDOW_SUFFIX_NAME));
                        router_acc.replace_route("/loading", None, None);
                    }
                    Err(warp::error::Error::InvalidLength { .. }) => {
                        error.set("Username length is invalid".into())
                    }
                    Err(_) => error.set("Unexpected error has occurred".into()),
                }
            }
        }
    };
    let mut new_account2 = new_account.clone();

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
                    div{
                        class: "photo-picker",                  
                        if profile_picture_is_empty {
                                rsx! {
                                    Icon {
                                        icon: Shape::User,
                                        size: 30,
                                    }
                                }
                            } else {
                                rsx!{
                                    img {
                                        class: "profile_photo",
                                        src: "{profile_picture_state}",
                                        height: "100",
                                        width: "100",
                                    }
                                }
                            }
                        Button {
                            icon: Shape::Plus,
                            on_pressed: move |_| {
                                let path = match FileDialog::new().add_filter("image", &["jpg", "png", "jpeg", "svg"]).set_directory(".").pick_file() {
                                    Some(path) => path,
                                    None => return
                                };
                                let file = match std::fs::read(&path) {
                                    Ok(image_vec) => image_vec,
                                    Err(_) => vec![],
                                };
                                let filename = std::path::Path::new(&path)
                                .file_name()
                                .unwrap_or_else(|| std::ffi::OsStr::new(""))
                                .to_str()
                                .unwrap()
                                .to_string();
                                let parts_of_filename: Vec<&str> = filename.split('.').collect();

                                let mime = match parts_of_filename.last() {
                                    Some(m) => {
                                        match *m {
                                            "png" => IMAGE_PNG.to_string(),
                                            "jpg" => IMAGE_JPEG.to_string(),
                                            "jpeg" => IMAGE_JPEG.to_string(),
                                            "svg" => IMAGE_SVG.to_string(),
                                            &_ => "".to_string(),
                                        }
                                    },
                                    None =>  "".to_string(),
                                };
                                let image = match &file.len() {
                                    0 => "".to_string(),
                                    _ => {
                                        let prefix = format!("data:{};base64,", mime);
                                        let base64_image = base64::encode(&file);
                                        let img = prefix + base64_image.as_str();
                                        img
                                    }
                                };
                                profile_picture_state.set(image);
                            }
                        },
                    }

                    div { class: "m-bottom" },
                    div {
                        class: "full-width",
                        Input {
                            icon: Shape::Identification,
                            value: username.clone().to_string(),
                            placeholder: String::from("Choose a username.."),
                            on_change: move | evt: FormEvent | {
                                error.set(String::from(""));
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
                        },
                        Button {
                            icon: Shape::Check,
                            text: String::from("Recover Account"),
                            on_pressed:  move |_| router.clone().replace_route("/restore", None, None),
                        }
                    }
                }
            }
        }
    })
}
