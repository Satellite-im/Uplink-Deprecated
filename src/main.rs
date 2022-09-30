use clap::Parser;
use dioxus::desktop::tao;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

use dioxus::router::{Route, Router};
use dioxus::{desktop::tao::dpi::LogicalSize, prelude::*};
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
pub mod themes;
pub mod utils;

#[cfg(target_os = "macos")]
use tao::platform::macos::{CustomMenuItemExtMacOS, NativeImage};
use tao::{
  accelerator::{Accelerator, SysMods},
  clipboard::Clipboard,
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::KeyCode,
  menu::{MenuBar as Menu, MenuItem, MenuItemAttributes, MenuType},
  window::WindowBuilder,
};
mod state;

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
    #[clap(long)]
    experimental_node: bool,
}

fn main() {
    if fdlimit::raise_fd_limit().is_none() {}

    let mut main_menu = Menu::new();
    let mut app_menu = Menu::new();
    let mut edit_menu = Menu::new();
    let mut window_menu = Menu::new();

    app_menu.add_native_item(MenuItem::Quit);
    app_menu.add_native_item(MenuItem::About("WarpGUI".to_string()));
    // add native shortcuts to `edit_menu` menu
    // in macOS native item are required to get keyboard shortcut
    // to works correctly
    edit_menu.add_native_item(MenuItem::Undo);
    edit_menu.add_native_item(MenuItem::Redo);
    edit_menu.add_native_item(MenuItem::Separator);
    edit_menu.add_native_item(MenuItem::Cut);
    edit_menu.add_native_item(MenuItem::Copy);
    edit_menu.add_native_item(MenuItem::Paste);
    edit_menu.add_native_item(MenuItem::SelectAll);

    window_menu.add_native_item(MenuItem::Minimize);
    window_menu.add_native_item(MenuItem::Zoom);
    window_menu.add_native_item(MenuItem::Separator);
    window_menu.add_native_item(MenuItem::ShowAll);
    window_menu.add_native_item(MenuItem::EnterFullScreen);
    window_menu.add_native_item(MenuItem::Separator);
    window_menu.add_native_item(MenuItem::CloseWindow);

    main_menu.add_submenu("Warp GUI", true, app_menu);
    main_menu.add_submenu("Edit", true, edit_menu);
    main_menu.add_submenu("Window", true, window_menu);
    let opt = Opt::parse();

    if let Some(path) = opt.path {
        *DEFAULT_PATH.write() = path;
    }

    let file_appender = tracing_appender::rolling::hourly(DEFAULT_PATH.read().join("logs"), "warp-gui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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
        opt.experimental_node,
    )) {
        Ok((i, c)) => (Account(i.clone()), Messaging(c.clone())),
        Err(_e) => todo!(),
    };


    let window = WindowBuilder::new()
        .with_title(DEFAULT_WINDOW_NAME.read().clone())
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(1200.0, 730.0))
        .with_menu(main_menu);

    dioxus::desktop::launch_with_props(
        App,
        State {
            tesseract,
            account,
            messaging,
        },
        |c| {
            c.with_window(|_| window.into())
        },
    );
}

async fn initialization(
    path: PathBuf,
    tesseract: Tesseract,
    experimental: bool,
) -> Result<
    (
        Arc<RwLock<Box<dyn MultiPass>>>,
        Arc<RwLock<Box<dyn RayGun>>>,
    ),
    warp::error::Error,
> {
    let config = MpIpfsConfig::production(&path, experimental);

    let account = warp_mp_ipfs::ipfs_identity_persistent(config, tesseract, None)
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
