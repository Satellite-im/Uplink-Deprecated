use dioxus::prelude::*;
mod extension;
use extension::{BasicExtension, Extension, ExtensionType};

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
pub extern "C" fn ret_rend() -> Box<fn(Scope) -> Element> {
    Box::new(HelloExtension::render)
}

#[no_mangle]
pub extern "C" fn ret_info() -> Box<Extension> {
    Box::new(HelloExtension::info())
}
