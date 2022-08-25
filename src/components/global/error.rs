use std::sync::Arc;

use dioxus::prelude::*;
use sir::css;
use warp::{tesseract::Tesseract, sync::RwLock, multipass::MultiPass};
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{components::ui_kit::loader::Loader, TESSERACT, MULTIPASS, LANGUAGE};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    error: String,
}

#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let css = css!("
        max-width: 350px;
        position: relative;
    ");

    let parent_css = css!("
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
        height: 80%;
    ");

    cx.render(rsx!{
        div {
            class: "{parent_css}",
            div {
                class: "{css}",
                h1: { 
                    "Uh-oh 💀"
                },
                p: {
                    "{cx.props.error}"
                }
            }
        }
    })
}