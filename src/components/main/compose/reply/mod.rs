use dioxus::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props {
    message: String,
    is_remote: bool,
}

#[allow(non_snake_case)]
pub fn Reply(cx: Scope<Props>) -> Element {
    let class = if cx.props.is_remote {
        "remote"
    } else {
        "local"
    };


    let box_right = "🭽";
    let box_left = "🭾";

    #[cfg(target="macos")]
    let box_left = "⎤";

    #[cfg(target="windows")]
    let box_left = "⎤";

    #[cfg(target="macos")]
    let box_right = "⎡";

    #[cfg(target="windows")]
    let box_right = "⎡";

    cx.render({
        rsx! {
            div {
                class: "reply {class}",
                (cx.props.is_remote).then(|| rsx! {
                    p {
                        class: "box-drawing left",
                        "{box_right}"
                    }
                }),
                (!cx.props.is_remote).then(|| rsx! {
                    div {
                        class: "pfp",
                    }
                }),
                p {
                    "{cx.props.message}",
                },
                (cx.props.is_remote).then(|| rsx! {
                    div {
                        class: "pfp",
                    }
                }),
                (!cx.props.is_remote).then(|| rsx! {
                    span {
                        class: "box-drawing",
                        "{box_left}"
                    }
                })
            }
        }
    })
}
