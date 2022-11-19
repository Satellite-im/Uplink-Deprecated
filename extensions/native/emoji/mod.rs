use dioxus::prelude::*;
use sir::global_css;

use super::{Extension, ExtensionMeta, ExtensionType};

pub struct EmojiSelector;

impl BasicExtension for EmojiSelector {
    fn info(&self) -> Extension {
        Extension {
            name: String::from("Emoji Picker"),
            author: String::from("matt@satellite.im"),
            description: String::from(
                "Select emoji's from an organized list of all supported emojis. Also provides tooling to transcribe text names into emoji icons.",
            ),
            location: ExtensionType::ChatbarWidget,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        global_css!(
            "
            .emoji_picker-main {
                background-color: var(--theme-primary);

                .widget_CTA {
                    width: 40px;
                    height: 40px;
                    background: green;
                    border-radius: 20px;
                }

                .widget_body {
                    display: none;
                }
            }
        "
        );

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
