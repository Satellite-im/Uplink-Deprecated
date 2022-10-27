use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use crate::components::ui_kit::textarea::TextArea;

// `text` is passed in this way because it is lifted. This allows for a 'send' button to clear the text
#[inline_props]
#[allow(non_snake_case)]
pub fn IconTextArea<'a>(
    cx: Scope,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    icon: Shape,
    placeholder: String,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "icon-input",
            Icon {
                icon: *icon,
            },
            TextArea {
                on_submit: |val| on_submit.call(val),
                text: text.clone(),
                placeholder: placeholder.to_string()
            },
        }
    })
}
