use std::path::PathBuf;
use std::sync::Arc;

use dioxus::prelude::*;
use language::{AvailableLanguages, Language};
use sir::AppStyle;
use warp::multipass::MultiPass;
use warp::raygun::RayGun;
use warp::sync::RwLock;
use warp::tesseract::Tesseract;

use crate::components::prelude::{auth, unlock};
use crate::components::ui_kit;

pub mod components;
pub mod language;
pub mod themes;

#[derive(PartialEq, Props)]
pub struct State {
    locked: bool,
}

static TESSERACT: AtomRef<Tesseract> = |_| Tesseract::default();
static LANGUAGE: AtomRef<Language> = |_| Language::by_locale(AvailableLanguages::EnUS);
static MULTIPASS: AtomRef<Option<Arc<RwLock<Box<dyn MultiPass>>>>> = |_| None;
static RAYGUN: AtomRef<Option<Arc<RwLock<Box<dyn RayGun>>>>> = |_| None;
static DEFAULT_PATH: AtomRef<PathBuf> = |_| PathBuf::from("./.cache");

fn main() {
    dioxus::desktop::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    // Loads the styles for all of our UIKit elements.
    let styles = ui_kit::build_style_tag();

    cx.render(rsx! (
        rsx!{
            style {
                "{styles}"
            },
            AppStyle {},
            Router {
                Route { to: "/", unlock::Unlock { pin: String::from("")} }
                Route { to: "/auth", auth::Auth { has_account: false } },
            }
        }
    ))
}