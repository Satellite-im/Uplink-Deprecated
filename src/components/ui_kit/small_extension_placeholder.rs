use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn SmallExtensionPlaceholder(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "mini-extension-renderer",
            button {
                "+"
            }
        }
    })
}
