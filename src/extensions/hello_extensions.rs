use dioxus::prelude::*;
use sir::global_css;

use super::{Extension, ExtensionMeta, ExtensionType};

pub struct HelloExtension;

impl BasicExtension for HelloExtension {
    fn info(&self) -> Extension {
        Extension {
            name: String::from("Hello Extension"),
            author: String::from("matt@satellite.im"),
            description: String::from("Simple \"Hello, world!\" for extensions."),
            location: ExtensionType::SidebarWidget,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        // Define custom SCSS for your element. Be sure to keep your styles in scope.
        // Use a unique prefix specific to your extension to prevent conflicts.
        global_css!("
            .hello_ext-main {
                background-color: var(--theme-primary);
            }
        ");

        // Return your rendered element.
        cx.render(rsx! {
            div {
                class: "hello_ext-main",
                "Hello, world!"
            }
        })
    }
}
