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
            background: var(--theme-secondary);
            border-radius: 8px;
            transition: .2s;
        }
        .pin span.active {
            background: var(--theme-primary);
        }
        .pin span.error {
            background: var(--theme-red);
        }"
    }

    let mut active_or_error = "active";
    if cx.props.error {
        active_or_error = "error";
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
        }
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
                },
                span {
                    class: "{classes.4}"
                }
                
            }
        } 
    } else {
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