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
        let some_costly_data = use_state("Something.");

        // This render will run any time things within this scope change.
        // Some times this is not ideal and we should update data deterministically.
        // This example shows a simple way to update data every 3000ms (based on system clock)
        let last_update = use_state(date::timestamp);
        if utils::RenderEvery(3000, last_render.write()) {
            // Get some costly data and update.
            some_costly_data.set("Something else.")
        }

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
