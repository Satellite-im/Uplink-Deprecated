use dioxus::prelude::*;
use utils::extensions::{BasicExtension, ExtensionInfo, ExtensionType};

pub struct HelloExtension;

impl BasicExtension for HelloExtension {
    fn info() -> ExtensionInfo {
        ExtensionInfo {
            name: String::from("Hello Extension"),
            author: String::from("matt@satellite.im"),
            description: String::from("Simple \"Hello, world!\" for extensions."),
            location: ExtensionType::SidebarWidget,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        cx.render(rsx! {
            div { "Hello world!" }
        })

    }
}


#[no_mangle]
pub extern "C"
fn ret_rend() -> Box<fn(Scope) -> Element> {
    Box::new(HelloExtension::render)
}

#[no_mangle]
pub extern "C" fn ret_info() -> Box<ExtensionInfo>{
    Box::new(HelloExtension::info())
}