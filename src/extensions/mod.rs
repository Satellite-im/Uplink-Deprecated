mod stickers;

use dioxus::prelude::{Element, Scope};


pub enum ExtensionType {
    SidebarWidget,
    ChatbarIcon,
}

pub struct ExtensionMeta {
    name: String,
    author: String,
    description: String,
    location: ExtensionType,
}

impl Default for ExtensionMeta {
    fn default() -> Self {
        Self { name: Default::default(), author: Default::default(), description: Default::default(), location: ExtensionType::SidebarWidget }
    }
}

pub trait Extension {
    fn info(&self) -> ExtensionMeta;
    fn render(cx: Scope) -> Element;
}