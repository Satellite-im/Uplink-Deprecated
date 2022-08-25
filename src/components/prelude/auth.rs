use std::sync::Arc;

use dioxus::prelude::*;
use sir::css;
use warp::{tesseract::Tesseract, sync::RwLock, multipass::MultiPass};
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{components::ui_kit::loader::Loader, TESSERACT, MULTIPASS, LANGUAGE};

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
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let tess = use_atom_ref(&cx, TESSERACT);
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let tess = tess.read().clone();
    let mp = use_future(&cx, (&tess,), |(tess,)| async move {
        warp_mp_ipfs::ipfs_identity_persistent(
            MpIpfsConfig::production("./.cache"),
            tess,
            None,
        ).await.map(|mp| Arc::new(RwLock::new(Box::new(mp) as Box<dyn MultiPass>)))
    });

    let account_fetch_status = match mp.value() {
        Some(Ok(val)) => {
            multipass.set(Some(val));
            true
        },
        Some(Err(err)) => {
            // TODO: Make an error page and reroute there
            false
        },
        None => {
            false
        },
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
                if account_fetch_status {
                    label {
                        "hmm"
                    }
                } else {
                    Loader {
                        text: l.checking_account
                    }
                }
            }
        }
    })
}