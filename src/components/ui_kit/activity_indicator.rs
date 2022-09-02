use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct Props {
    inline: bool,
}

#[allow(non_snake_case)]
pub fn ActivityIndicator(cx: Scope<Props>) -> Element {
    let main_class = match cx.props.inline {
        true => "inline",
        false => "icon-icon",
    };

    cx.render(rsx! {
        div {
            class: "activity {main_class}",
            div { class: "bubble" },
            p {
                "Unknown"
            }
        }
    })
}
