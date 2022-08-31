use crate::{components::main::compose::Compose, main::sidebar::Sidebar, STATE};
use dioxus::prelude::*;
use sir::global_css;
use warp::{raygun::Conversation, tesseract::Tesseract};

pub mod compose;
pub mod sidebar;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    let conversation = match state.read().chat.clone() {
        Some(c) => c,
        None => Conversation::default(),
    };

    // Start UI
    global_css! {"
    .main {
        display: flex;
        text-align: center;
        width: 100%;
        height: 100%;
        flex-direction: row;
    }
    "}

    cx.render(rsx! {
        div {
            class: "main",
            Sidebar {},
            Compose {
                conversation: conversation
            },
        }
    })
}
