use dioxus::prelude::*;
use sir::global_css;

use super::{Extension, ExtensionMeta, ExtensionType};

pub struct StickersExtension;

impl Extension for StickersExtension {
    fn info(&self) -> ExtensionMeta {
        ExtensionMeta {
            name: String::from("Stickers"),
            author: String::from("matt@satellite.im"),
            description: String::from("Enables support for satellite provided stickers."),
            location: ExtensionType::ChatbarIcon,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        global_css!(
            "
            .ext-stickers {
                background-color: var(--theme-primary);
            }
        "
        );
        cx.render(rsx! {
            div {
                class: "ext-stickers",
            }
        })
    }
}
