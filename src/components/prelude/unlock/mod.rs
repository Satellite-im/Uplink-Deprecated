use dioxus::core::to_owned;
use dioxus::router::use_router;
use dioxus::{events::KeyCode, prelude::*};
use dioxus_heroicons::outline::Shape;
use sir::css;
use ui_kit::{
    button::{self, Button},
    pin::Pin,
    tooltip::{ArrowPosition, Tooltip},
};
use warp::tesseract::Tesseract;

use crate::LANGUAGE;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct UnlockProps {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Unlock(cx: Scope<UnlockProps>) -> Element {
    log::debug!("rendering Unlock");
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();
    let router = use_router(&cx).clone();
    let router2 = router.clone();
    let router3 = router.clone();

    let pin = use_state(&cx, String::new);
    let show_tip = use_state(&cx, || false);
    let error = use_state(&cx, String::new);
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };
    let auth_text = l.auth_tooltip.clone();

    let confirm_button_class = if error.is_empty() {
        "confirm-button"
    } else {
        "confirm-button has-error"
    };

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
                    show_tip.then(||
                        rsx! {
                            span {
                                class: "{confirm_button_class}",
                                Button {
                                    icon: if error.is_empty() {
                                        Shape::Check
                                    } else {
                                        Shape::XMark
                                    }
                                    on_pressed: move |_| {
                                        let tesseract = cx.props.tesseract.clone();
                                        match tesseract.unlock(pin.as_bytes()) {
                                            Ok(_) => {
                                                router.replace_route("/loading", None, None)
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
                show_tip.then(||
                rsx! {
                    span {
                        class: "pin_tooltip",
                        Tooltip {
                            text: auth_text,
                            arrow_position: ArrowPosition::Top
                        }
                    }
                }),
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
                        if evt.value.len() <= 6 {
                            pin.set(evt.value.to_string());
                        } else {
                            //Because we exceeded 6, we want to show the tooltip showing the error
                            show_tip.set(true);
                            //This will spawn the background task as kind of a "timeout" for "show_tip" state
                            cx.spawn({
                                // this is the equiv if `let show_tip = show_tip.clone()`
                                to_owned![show_tip];
                                async move {
                                    // since we are using `async` we want to avoid using `std::thread::sleep` as it would stall all
                                    // running task. Instead, rely on internal functions from either tokio or futures to
                                    // delay for a set duration
                                    tokio::time::sleep(std::time::Duration::from_secs(4)).await;
                                    show_tip.set(false);
                                }
                            });
                            pin.set(evt.value[..6].to_string());
                        }

                        // If tesseract exists, we can try to unlock as we type to save time
                        // We can ignore the error though since we're doing this without the users command
                        if evt.value.len() >= 4 && tesseract_available {
                            let tesseract = cx.props.tesseract.clone();
                            if tesseract.unlock(evt.value.as_ref()).is_ok() {
                                router2.replace_route("/loading", None, None)
                            }
                        }
                    },
                    onkeyup: move |evt| {
                        if evt.key_code == KeyCode::Enter {
                            if pin.len() < 4 && !tesseract_available {
                                error.set(l.short_pin.clone());
                            } else {
                                let tesseract = cx.props.tesseract.clone();
                                match tesseract.unlock(pin.as_bytes()) {
                                    Ok(_) => router3.replace_route("/loading", None, None),
                                    Err(_) => error.set(l.invalid_pin.clone()),
                                }
                            }
                        }
                    },
                }
            },
            div {
                class: "login-actions",
                Button {
                    icon: Shape::User,
                    disabled: true,
                    state: button::State::Secondary,
                    on_pressed: move |_| {},
                },
                Button {
                    icon: Shape::GlobeAlt,
                    disabled: true,
                    state: button::State::Secondary,
                    on_pressed: move |_| {},
                },
            }
        },
    })
}
