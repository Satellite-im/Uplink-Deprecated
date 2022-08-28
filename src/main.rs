use std::path::PathBuf;
use std::sync::Arc;

use dioxus::desktop::tao::dpi::LogicalSize;
use dioxus_toast::ToastManager;
use dioxus::prelude::*;
use language::{AvailableLanguages, Language};
use once_cell::sync::Lazy;
use sir::AppStyle;
use warp::multipass::MultiPass;
use warp::raygun::RayGun;
use warp::sync::RwLock;
use warp::tesseract::Tesseract;

use crate::components::prelude::{auth, unlock};
use crate::components::{ui_kit, main};

pub mod components;
pub mod language;
pub mod themes;

#[derive(PartialEq, Props)]
pub struct State {
    tesseract: Tesseract,
}

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();
static LANGUAGE: AtomRef<Language> = |_| Language::by_locale(AvailableLanguages::EnUS);
static MULTIPASS: AtomRef<Option<Arc<RwLock<Box<dyn MultiPass>>>>> = |_| None;
static RAYGUN: AtomRef<Option<Arc<RwLock<Box<dyn RayGun>>>>> = |_| None;
static DEFAULT_PATH: Lazy<RwLock<PathBuf>> = Lazy::new(|| RwLock::new(PathBuf::from("./.cache")));

fn main() {
    let tesseract = match Tesseract::from_file(DEFAULT_PATH.read().join(".keystore")) {
        Ok(tess) => tess,
        Err(_) => {
            //doesnt exist so its set
            let mut tess = Tesseract::default();
            tess.set_file(DEFAULT_PATH.read().join(".keystore"));
            tess.set_autosave();
            tess
        }
    };

    dioxus::desktop::launch_with_props(App, State { tesseract }, |c| {
        c.with_window(|w| {
            w
                .with_title("Warp by Satellite")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(900.0, 600.0))
        })
    });
}

#[allow(non_snake_case)]
fn App(cx: Scope<State>) -> Element {
    // Loads the styles for all of our UIKit elements.
    let styles = ui_kit::build_style_tag();
    let toast = use_atom_ref(&cx, TOAST_MANAGER);
    cx.render(rsx! (
        rsx!{
            style {
                "{styles}"
            },
            dioxus_toast::ToastFrame {
                manager: toast,
            }
            AppStyle {},
            Router {
                Route { to: "/", unlock::Unlock { tesseract: cx.props.tesseract.clone() } }
                Route { to: "/auth", auth::Auth { tesseract: cx.props.tesseract.clone() } },
                Route { to: "/main", main::Main { tesseract: cx.props.tesseract.clone() } },
            }
        }
    ))
}
