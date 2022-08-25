use std::sync::Arc;

use dioxus::prelude::*;
use sir::css;
use warp::{tesseract::Tesseract, sync::RwLock, multipass::MultiPass};
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{components::ui_kit::loader::Loader, TESSERACT, MULTIPASS};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    has_account: bool,
}

struct TessHolder(Tesseract);

impl AsRef<Tesseract> for TessHolder {
    fn as_ref(&self) -> &Tesseract {
        &self.0
    }
}

impl PartialEq for TessHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_unlock() == other.0.is_unlock()
    }
}

#[allow(non_snake_case)]
pub fn Auth(cx: Scope<Props>) -> Element {
    let tess = use_atom_ref(&cx, TESSERACT);
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let tesseract = TessHolder(tess.read().clone());
    let mp = use_future(&cx, &(&tesseract,), |(&tesseract,)| async move {
        warp_mp_ipfs::ipfs_identity_persistent(
            MpIpfsConfig::production("./.cache"),
            tesseract.as_ref().clone(),
            None,
        ).await
    });

    multipass.set(Some(Arc::new(RwLock::new(Box::new(mp)))));

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