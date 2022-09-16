use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;

use dioxus_desktop::{Config, WindowBuilder};
use dioxus_desktop::tao::dpi::LogicalSize;
use dioxus::prelude::*;
use fermi::prelude::*;
use dioxus_router::{Route, Router};
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
use warp_mp_ipfs::config::MpIpfsConfig;
use warp_rg_ipfs::config::RgIpfsConfig;
use warp_rg_ipfs::Persistent;

use crate::components::main;
use crate::components::prelude::{auth, unlock};

pub mod components;
pub mod extensions;
pub mod language;
mod state;
pub mod themes;

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();
static LANGUAGE: AtomRef<Language> = |_| Language::by_locale(AvailableLanguages::EnUS);
static DEFAULT_PATH: Lazy<RwLock<PathBuf>> = Lazy::new(|| RwLock::new(PathBuf::from("./.cache")));
pub const WINDOW_SUFFIX_NAME: &'static str = "Warp GUI";
static DEFAULT_WINDOW_NAME: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(String::from(WINDOW_SUFFIX_NAME)));
static STATE: AtomRef<PersistedState> = |_| PersistedState::load_or_inital();

#[derive(PartialEq, Props)]
pub struct State {
    tesseract: Tesseract,
    account: Account,
    messaging: Messaging,
}

#[derive(Debug, Parser)]
#[clap(name = "")]
struct Opt {
    #[clap(long)]
    path: Option<PathBuf>,
    #[clap(long)]
    title: Option<String>,
}

fn main() {
    if fdlimit::raise_fd_limit().is_none() {}

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

    let (account, messaging) = match warp::async_block_in_place_uncheck(initialization(
        DEFAULT_PATH.read().clone(),
        tesseract.clone(),
    )) {
        Ok((i, c)) => (Account(i.clone()), Messaging(c.clone())),
        Err(_e) => todo!(),
    };

    dioxus_desktop::launch_with_props(
        App,
        State {
            tesseract,
            account,
            messaging,
        },
        
            Config::default().with_window(
                WindowBuilder::default().with_title(DEFAULT_WINDOW_NAME.read().clone())
                    .with_resizable(true)
                    .with_inner_size(LogicalSize::new(1200.0, 730.0))
            )
        ,
    );
}

async fn initialization(
    path: PathBuf,
    tesseract: Tesseract,
) -> Result<
    (
        Arc<RwLock<Box<dyn MultiPass>>>,
        Arc<RwLock<Box<dyn RayGun>>>,
    ),
    warp::error::Error,
> {
    let account = warp_mp_ipfs::ipfs_identity_persistent(
        MpIpfsConfig::production(path.clone()),
        tesseract,
        None,
    )
    .await
    .map(|mp| Arc::new(RwLock::new(Box::new(mp) as Box<dyn MultiPass>)))?;

    let messenging = warp_rg_ipfs::IpfsMessaging::<Persistent>::new(
        Some(RgIpfsConfig::production(path)),
        account.clone(),
        None,
    )
    .await
    .map(|rg| Arc::new(RwLock::new(Box::new(rg) as Box<dyn RayGun>)))?;

    Ok((account, messenging))
}

#[allow(non_snake_case)]
fn App(cx: Scope<State>) -> Element {
    //TODO: Display an error instead of panicing
    std::fs::create_dir_all(DEFAULT_PATH.read().clone()).expect("Error creating directory");
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
        },
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
            Route { to: "/auth", auth::Auth { account: cx.props.account.clone() } },
            Route { to: "/main", main::Main { account: cx.props.account.clone(), messaging: cx.props.messaging.clone() } },
        }
    ))
}

#[derive(Clone)]
pub struct Account(Arc<RwLock<Box<dyn MultiPass>>>);

impl Deref for Account {
    type Target = Arc<RwLock<Box<dyn MultiPass>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_locked() == other.0.is_locked()
    }
}

#[derive(Clone)]
pub struct Messaging(Arc<RwLock<Box<dyn RayGun>>>);

impl Deref for Messaging {
    type Target = Arc<RwLock<Box<dyn RayGun>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Messaging {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_locked() == other.0.is_locked()
    }
}
