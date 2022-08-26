use dioxus::{events::KeyCode, prelude::*};
use dioxus_heroicons::outline::Shape;
use sir::{css, global_css};
use warp::tesseract::Tesseract;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        pin::Pin,
    },
    DEFAULT_PATH, LANGUAGE,
};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct UnlockProps {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Unlock(cx: Scope<UnlockProps>) -> Element {
    // let tess = use_atom_ref(&cx, TESSERACT);
    // let default_path = use_atom_ref(&cx, DEFAULT_PATH);

    //TODO: Display an error instead of panicing
    std::fs::create_dir_all(DEFAULT_PATH.read().clone()).expect("Error creating directory");

    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();

    global_css! {"
        .login-actions {
            div {
                padding: 0.2rem 0.6rem;
            }
        }
    "}

    let css = css! { "max-width: 350px;position: relative;" };

    let login_actions_css = css! {"
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        left: 2rem;
        max-height: 40px;
        display: inline-block;
        text-align: right;
        z-index: 3;
    "};

    let parent_css = css! {"
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
        height: 80%;
    "};

    let invis_input = css! {"
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        z-index: 2;
        cursor: default;
        opacity: 0;
        font-size: 0;
    "};

    let confirm_button = css! {"
        position: absolute;
        right: -80px;
        bottom: -12px;
        disabled: true;
        z-index: 3;
    "};

    let pin = use_state(&cx, || String::from(""));
    let error = use_state(&cx, || String::from(""));
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };
    let valid_pin = pin.len() >= 4;
    // Used later to try to unlock as we type a valid pin automatically much like modern phones and operating systems.

    // if default_path.read().join(".keystore").exists() {
    //     tess.set(
    //         match Tesseract::from_file(default_path.read().join(".keystore")) {
    //             Ok(tess) => tess, //tesseract exist and is loaded
    //             Err(_) => {
    //                 //doesnt exist so its set
    //                 let mut tess = Tesseract::default();
    //                 tess.set_file(default_path.read().join(".keystore"));
    //                 tess.set_autosave();
    //                 tess
    //             }
    //         },
    //     );
    // } else {
    //     tess.write().set_file(default_path.read().join(".keystore"));
    //     tess.write().set_autosave();
    // }
    let mut tesseract = cx.props.tesseract.clone();

    let tesseract_available = tesseract.exist("keypair");
    cx.render(rsx! {
        div {
            class: "{parent_css}",
            div {
                class: "{css}",
                h2 {
                    "{l.create_pin}",
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
                                class: "{confirm_button}",
                                IconButton {
                                    icon: if error.is_empty() {
                                        Shape::Check
                                    } else {
                                        Shape::X
                                    }
                                    onclick: move |_| {
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
                    class: "{invis_input}",
                    value: "{pin}",
                    autofocus: "true",
                    onkeypress: move |evt| {
                        error.set(String::from(""));
                        if evt.key_code == KeyCode::Enter {
                            if pin.len() < 4 {
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
                    oninput: move |evt| {
                        pin.set(evt.value.clone());
                        // If tesseract exists, we can try to unlock as we type to save time
                        // We can ignore the error though since we're doing this without the users command
                        if pin.len() >= 4 && tesseract_available {
                            let mut tesseract = cx.props.tesseract.clone();
                            match tesseract.unlock(pin.as_bytes()) {
                                Ok(_) => use_router(&cx).push_route("/auth", None, None),
                                Err(_) => {},
                            }
                        }
                        // If the pin entered is longer than the allowed limit, we'll just set it back to the max.
                        if pin.len() > 6 {
                            pin.set(evt.value[..6].to_string());
                        }
                    },
                }
            },
        },
        div {
            class: "login-actions {login_actions_css}",
            IconButton {
                icon: Shape::User,
                disabled: true,
                state: icon_button::State::Secondary,
                onclick: move |_| {},
            },
            IconButton {
                icon: Shape::GlobeAlt,
                disabled: true,
                state: icon_button::State::Secondary,
                onclick: move |_| {},
            },
        }
    })
}
