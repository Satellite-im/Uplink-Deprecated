use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {
    message: String,
}

#[allow(non_snake_case)]
pub fn Reply(cx: Scope<Props>) -> Element {
    cx.render({        
        rsx! {
            div {
                class: "reply",
                p {
                    "{cx.props.message}",
                }
            }
        }
    })
}
