use dioxus::prelude::*;
use utils::extensions::Extension;

#[derive(Props, PartialEq)]
pub struct Props {
    extension: Extension,
}

#[allow(non_snake_case)]
pub fn ExtensionOptions(cx: Scope<Props>) -> Element {
    log::debug!("rendering extension settings");

    cx.render(rsx! {
        div {
            class: "extension",
            "{cx.props.extension.name}",
            "{cx.props.extension.author}",
            "{cx.props.extension.description}"
        }
    })
}
