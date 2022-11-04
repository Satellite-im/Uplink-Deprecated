use dioxus::prelude::*;
use sir::{global_css};
mod extension;
use extension::{Extension, BasicExtension, ExtensionType};

pub struct HelloExtension;

impl BasicExtension for HelloExtension {
    fn info() -> Extension {
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
            },
        })
    }
}


#[no_mangle]
pub extern "C" 
fn ret_rend() -> Box<fn(Scope) -> Element> {
    Box::new(HelloExtension::render)
}

#[no_mangle]
pub extern "C" fn ret_info() -> Box<Extension>{
    Box::new(HelloExtension::info())
}