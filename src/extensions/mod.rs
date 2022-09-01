use dioxus::prelude::{Element, Scope};

pub enum ExtensionType {
    SidebarWidget,
    ChatbarIcon,
}

#[allow(dead_code)]
pub struct Extension {
    name: String,
    author: String,
    description: String,
    location: ExtensionType,
}

impl Default for Extension {
    fn default() -> Self {
        Self {
            name: Default::default(),
            author: Default::default(),
            description: Default::default(),
            location: ExtensionType::SidebarWidget,
        }
    }
}

pub trait BasicExtension {
    fn info(&self) -> Extension;
    fn render(cx: Scope) -> Element;
}
