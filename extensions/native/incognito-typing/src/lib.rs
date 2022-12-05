use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape};
use state::STATE;
use utils::extensions::{BasicExtension, ExtensionInfo, ExtensionType};
use ui_kit::{button::{self, Button}};

pub struct ExtIncognitoTyping;

impl BasicExtension for ExtIncognitoTyping {
    fn info() -> ExtensionInfo {
        ExtensionInfo {
            name: String::from("Incognito Typing"),
            author: String::from("matt@satellite.im"),
            description: String::from("Incognito Typing allows you to disable the typing notification in chat"),
            location: ExtensionType::ChatbarIcon,
        }
    }

    fn render(cx: Scope) -> Element {
        let state = use_atom_ref(&cx, STATE);
        let send_typing = state.read().send_typing;

        let handle_click = move |_| {
            state.write().send_typing = !send_typing;
        };

        cx.render(rsx! {
            div {
                id: "incognito-typing",
                Button {
                    on_pressed: handle_click,
                    state: button::State::Secondary,
                    icon: match &send_typing {
                        true => Shape::Eye,
                        false => Shape::EyeSlash,
                    }
                }
            }
        })
    }
}
