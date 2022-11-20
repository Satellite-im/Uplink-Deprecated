use dioxus::prelude::*;

pub enum ExtensionType {
    SidebarWidget,
    ChatbarIcon,
}


pub struct Extension {
    pub name: String,
    pub author: String,
    pub description: String,
    pub location: ExtensionType,
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
    fn info() -> Extension;
    fn render(cx: Scope) -> Element;
}