use dioxus::prelude::*;
use utils::extensions::{BasicExtension, ExtensionInfo, ExtensionType};

pub struct EmojiSelector;

impl BasicExtension for EmojiSelector {
    fn info() -> ExtensionInfo {
        ExtensionInfo {
            name: String::from("Emoji Picker"),
            author: String::from("matt@satellite.im"),
            description: String::from(
                "Select emoji's from an organized list of all supported emojis. Also provides tooling to transcribe text names into emoji icons.",
            ),
            location: ExtensionType::SidebarWidget,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        // Return your rendered element.
        cx.render(rsx! {
            div {
                class: "emoji_picker-main",
                div {
                    class: "widget_CTA",
                    "T"
                },
                div {
                    class: "widget_body",
                    "Body"
                }
            }
        })
    }
}
