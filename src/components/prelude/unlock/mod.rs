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

use crate::fl;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct UnlockProps {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Unlock(cx: Scope<UnlockProps>) -> Element {
    log::debug!("rendering Unlock");

    let pin = use_state(&cx, String::new);
    let show_tip = use_state(&cx, || false);
    let error = use_state(&cx, String::new);
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };

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
                    match tesseract_available {
                        true => [fl!("unlock-enter-pin")],
                        false => [fl!("unlock-create-pin")],
                    }
                },
                label {
                    match tesseract_available {
                        true => [fl!("unlock-enter-a-pin")],
                        false => [fl!("unlock-create-a-pin")],
                    }
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
                                                use_router(&cx).push_route("/loading", None, None)
                                            },
                                            Err(_) => error.set(fl!("unlock-invalid-pin"))
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
                            text: fl!("unlock-tooltip"),
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
                                use_router(&cx).push_route("/loading", None, None)
                            }
                        }
                    },
                    onkeyup: move |evt| {
                        if evt.key_code == KeyCode::Enter {
                            if pin.len() < 4 && !tesseract_available {
                                error.set(fl!("unlock-pin-too-short"));
                            } else {
                                let tesseract = cx.props.tesseract.clone();
                                match tesseract.unlock(pin.as_bytes()) {
                                    Ok(_) => use_router(&cx).push_route("/loading", None, None),
                                    Err(_) => error.set(fl!("unlock-invalid-pin")),
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
