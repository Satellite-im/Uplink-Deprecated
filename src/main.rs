use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use dioxus::desktop::tao::dpi::LogicalSize;
use dioxus::prelude::*;
use dioxus_toast::ToastManager;
use language::{AvailableLanguages, Language};
use once_cell::sync::Lazy;
use sir::AppStyle;
use state::PersistedState;
use themes::Theme;
use warp::multipass::MultiPass;
use warp::raygun::RayGun;
use warp::sync::RwLock;
use warp::tesseract::Tesseract;

use crate::components::main;
use crate::components::prelude::{auth, unlock};

pub mod components;
pub mod extensions;
pub mod language;
mod state;
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
pub const WINDOW_SUFFIX_NAME: &'static str = "Warp GUI";
static DEFAULT_WINDOW_NAME: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(String::from(WINDOW_SUFFIX_NAME)));
static STATE: AtomRef<PersistedState> = |_| PersistedState::load_or_inital();

#[derive(Debug, Parser)]
#[clap(name = "")]
struct Opt {
    #[clap(long)]
    path: Option<PathBuf>,
    #[clap(long)]
    title: Option<String>,
}

fn main() {
    let opt = Opt::parse();

    if let Some(path) = opt.path {
        *DEFAULT_PATH.write() = path;
    }

    if let Some(title) = opt.title {
        *DEFAULT_WINDOW_NAME.write() = title;
    }

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
            w.with_title(DEFAULT_WINDOW_NAME.read().clone())
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1200.0, 730.0))
        })
    });
}

#[allow(non_snake_case)]
fn App(cx: Scope<State>) -> Element {
    // Loads the styles for all of our UIKit elements.
    let theme_colors = Theme::load_or_default().rosetta();
    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    let css = include_str!(".styles.css");

    cx.render(rsx!(
        div {
            class: "markdown",
            dangerous_inner_html: "
                <link
                    rel=\"stylesheet\"
                    href=\"https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css\"
                />
            ",
        }
        style {
            "{theme_colors}",
            "{css}"
        },
        dioxus_toast::ToastFrame {
            manager: toast,
        }
        AppStyle {},
        Router {
            Route { to: "/", unlock::Unlock { tesseract: cx.props.tesseract.clone() } }
            Route { to: "/auth", auth::Auth { tesseract: cx.props.tesseract.clone() } },
            Route { to: "/main", main::Main { } },
        }
    ))
}
