use dioxus::prelude::*;
use sir::css;
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{components::ui_kit::loader::Loader, TESSERACT};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    has_account: bool,
}


#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let tess = use_atom_ref(&cx, TESSERACT);
    let mp = use_future(&cx, (), |_| async move {
        warp_mp_ipfs::ipfs_identity_persistent(
            MpIpfsConfig::production("./.cache"),
            tess.read().clone(),
            None,
        ).await
    });

    let status = match mp.value() {
        Some(Ok(val)) => "ready!",
        Some(Err(err)) => "errored!",
        None => "loading!",
    };

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
                Loader {
                    text: "Checking account..".to_string()
                }
            }
        }
    })
}