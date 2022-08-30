use dioxus::prelude::*;
use sir::global_css;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    pin: Vec<u8>,
    error: bool,
}

#[allow(non_snake_case)]
pub fn Pin(cx: Scope<Props>) -> Element {
    global_css! {"
        .pin {
            width: 100%;
            display: flex;
            justify-content: space-between;
            transition: .2s;
        }
        .pin span {
            display: inline-block;
            height: 16px;
            width: 16px;
            background: var(--theme-foreground);
            border-radius: 8px;
            transition: .2s;
        }
        .pin span.active {
            background: var(--theme-primary);
        }
        .pin span.error {
            background: var(--theme-red);
        }
        .shake {
            animation: shake 0.3s;
        }
        @keyframes shake {
            0% { transform: translate(1px, 1px) rotate(0deg); }
            10% { transform: translate(-1px, -2px) rotate(-1deg); }
            20% { transform: translate(-3px, 0px) rotate(1deg); }
            30% { transform: translate(3px, 2px) rotate(0deg); }
            40% { transform: translate(1px, -1px) rotate(1deg); }
            50% { transform: translate(-1px, 2px) rotate(-1deg); }
            60% { transform: translate(-3px, 1px) rotate(0deg); }
            70% { transform: translate(3px, 1px) rotate(-1deg); }
            80% { transform: translate(-1px, -1px) rotate(1deg); }
            90% { transform: translate(1px, 2px) rotate(0deg); }
            100% { transform: translate(1px, -2px) rotate(-1deg); }
        }
        "
    }

    let mut active_or_error = "active";
    let mut shake = "no-shake";

    if cx.props.error {
        active_or_error = "error";
        shake = "shake";
    }
    // TODO: clean this up with an iterator
    let classes = (
        if cx.props.pin.get(0).is_some() {
            active_or_error
        } else {
            "inactive"
        },
        if cx.props.pin.get(1).is_some() {
            active_or_error
        } else {
            "inactive"
        },
        if cx.props.pin.get(2).is_some() {
            active_or_error
        } else {
            "inactive"
        },
        if cx.props.pin.get(3).is_some() {
            active_or_error
        } else {
            "inactive"
        },
        if cx.props.pin.get(4).is_some() {
            active_or_error
        } else {
            "inactive"
        },
        if cx.props.pin.get(5).is_some() {
            active_or_error
        } else {
            "inactive"
        },
    );

    cx.render(if cx.props.pin.len() <= 4 {
        rsx! {
            div {
                class: "pin",
                span {
                    class: "{classes.0}"
                },
                span {
                    class: "{classes.1}"
                },
                span {
                    class: "{classes.2}"
                },
                span {
                    class: "{classes.3}"
                }
            }
        }
    } else if cx.props.pin.len() == 5 {
        rsx! {
            div {
                class: "pin {shake}",
                span {
                    class: "{classes.0}"
                },
                span {
                    class: "{classes.1}"
                },
                span {
                    class: "{classes.2}"
                },
                span {
                    class: "{classes.3}"
                },
                span {
                    class: "{classes.4}"
                }

            }
        }
    } else {
        rsx! {
            div {
                class: "pin {shake}",
                span {
                    class: "{classes.0}"
                },
                span {
                    class: "{classes.1}"
                },
                span {
                    class: "{classes.2}"
                },
                span {
                    class: "{classes.3}"
                },
                span {
                    class: "{classes.4}"
                },
                span {
                    class: "{classes.5}"
                }
            }
        }
    })
}
