use dioxus::router::use_router;
use dioxus::{events::KeyCode, prelude::*};
use dioxus_heroicons::outline::Shape;
use sir::css;
use warp::tesseract::Tesseract;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        pin::Pin,
    },
    LANGUAGE,
};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct UnlockProps {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Unlock(cx: Scope<UnlockProps>) -> Element {
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();

    let pin = use_state(&cx, || String::from(""));
    let error = use_state(&cx, || String::from(""));
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };
    let valid_pin = pin.len() >= 4;

    let tesseract_available = cx.props.tesseract.exist("keypair");

    cx.render(rsx! {
        div {
            class: "unlock",
            div {
                class: "container",
                h2 {
                    (tesseract_available).then(|| l.enter_pin.clone()),
                    (!tesseract_available).then(|| l.create_pin.clone()),
                },
                label {
                    (tesseract_available).then(|| l.enter_your_pin.clone()),
                    (!tesseract_available).then(|| l.choose_a_pin.clone()),
                },
                div {
                    class: "m-bottom-xl",
                },
                div {
                    style: "position: relative;",
                    Pin {
                        pin: pin.as_bytes().to_vec(),
                        error: !error.is_empty()
                    },
                    valid_pin.then(||
                        rsx! {
                            span {
                                class: "confirm-button",
                                IconButton {
                                    icon: if error.is_empty() {
                                        Shape::Check
                                    } else {
                                        Shape::X
                                    }
                                    on_pressed: move |_| {
                                        let mut tesseract = cx.props.tesseract.clone();
                                        match tesseract.unlock(pin.as_bytes()) {
                                            Ok(_) => {
                                                use_router(&cx).push_route("/auth", None, None)
                                            },
                                            Err(_) => error.set(l2.invalid_pin.clone()),
                                        }
                                    },
                                },
                            },
                        }
                    ),
                }
                div {
                    class: "m-bottom-xl",
                },
                p {
                    class: "{error_class}",
                    "{error}　"
                },
                input {
                    class: "invis-input",
                    value: "{pin}",
                    autofocus: "true",
                    oninput: move |evt| {
                        error.set(String::from(""));

                        // If the pin entered is longer than the allowed limit, ignore it.
                        if pin.len() < 6 {
                            pin.set(evt.value.to_string());
                        }

                        // If tesseract exists, we can try to unlock as we type to save time
                        // We can ignore the error though since we're doing this without the users command
                        if evt.value.len() >= 4 && tesseract_available {
                            let mut tesseract = cx.props.tesseract.clone();
                            if tesseract.unlock(evt.value.as_ref()).is_ok() {
                                use_router(&cx).push_route("/auth", None, None)
                            }
                        }
                    },
                    onkeyup: move |evt| {
                        if evt.key_code == KeyCode::Enter {
                            if pin.len() < 4 && !tesseract_available {
                                error.set(l.short_pin.clone());
                            } else {
                                let mut tesseract = cx.props.tesseract.clone();
                                match tesseract.unlock(pin.as_bytes()) {
                                    Ok(_) => use_router(&cx).push_route("/auth", None, None),
                                    Err(_) => error.set(l.invalid_pin.clone()),
                                }
                            }
                        }
                    },
                }
            },
            div {
                class: "login-actions",
                IconButton {
                    icon: Shape::User,
                    disabled: true,
                    state: icon_button::State::Secondary,
                    on_pressed: move |_| {},
                },
                IconButton {
                    icon: Shape::GlobeAlt,
                    disabled: true,
                    state: icon_button::State::Secondary,
                    on_pressed: move |_| {},
                },
            }
        },
    })
}
