use clap::Parser;
use core::time;
use dioxus::desktop::tao;
use dioxus_heroicons::outline::Shape;
use fluent::{FluentBundle, FluentResource};
use std::{
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
    thread,
};
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;
use ui_kit::context_menu::{ContextItem, ContextMenu};
use unic_langid::LanguageIdentifier;
use utils::Storage;

use crate::iutils::config::Config;
use ::utils::Account;
use dioxus::router::{Route, Router};
use dioxus::{desktop::tao::dpi::LogicalSize, prelude::*};
use dioxus_toast::ToastManager;
use language::{AvailableLanguages, Language};
use once_cell::sync::Lazy;
use sir::AppStyle;
use state::PersistedState;
use themes::Theme;

use warp::{
    constellation::Constellation, multipass::MultiPass, raygun::RayGun, sync::RwLock,
    tesseract::Tesseract,
};
use warp_fs_ipfs::config::FsIpfsConfig;
use warp_mp_ipfs::config::MpIpfsConfig;
use warp_rg_ipfs::config::RgIpfsConfig;
use warp_rg_ipfs::Persistent;

use crate::components::main;
use crate::components::prelude::{auth, loading, unlock};

pub mod components;
pub mod iui_kit;
pub mod iutils;
pub mod language;
pub mod themes;

use tao::window::WindowBuilder;

use tao::menu::{MenuBar as Menu, MenuItem};

mod state;

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();
static LANGUAGE: AtomRef<Language> = |_| Language::by_locale(AvailableLanguages::EnUS);

static DEFAULT_PATH: Lazy<RwLock<PathBuf>> =
    Lazy::new(|| RwLock::new(dirs::home_dir().unwrap_or_default().join(".warp")));
pub const WINDOW_SUFFIX_NAME: &str = "Uplink";

static DEFAULT_WINDOW_NAME: Lazy<RwLock<String>> =
    Lazy::new(|| RwLock::new(String::from(WINDOW_SUFFIX_NAME)));
static STATE: AtomRef<PersistedState> = |_| PersistedState::load_or_initial();

#[derive(PartialEq, Props)]
pub struct State {
    tesseract: Tesseract,
    account: Account,
    messaging: Messaging,
    storage: Storage,
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

    let ftl_string = match fs::read_to_string("src/language/en_US.ftl") {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file");
            // Exit the program with exit code `1`.
            String::from("")
        }
    };

    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    // TODO: Make this dynamic
    let loc: LanguageIdentifier = "en-US".parse().expect("Parsing failed.");
    let mut language = FluentBundle::new(vec![loc]);

    language
        .add_resource(&res)
        .expect("Failed to add FTL resources to the bundle.");

    let mut main_menu = Menu::new();
    let mut app_menu = Menu::new();
    let mut edit_menu = Menu::new();
    let mut window_menu = Menu::new();

    app_menu.add_native_item(MenuItem::Quit);
    app_menu.add_native_item(MenuItem::About(String::from("Uplink")));
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

    main_menu.add_submenu("Uplink", true, app_menu);
    main_menu.add_submenu("Edit", true, edit_menu);
    main_menu.add_submenu("Window", true, window_menu);

    let opt = Opt::parse();

    if let Some(path) = opt.path {
        *DEFAULT_PATH.write() = path;
    }

    let file_appender =
        tracing_appender::rolling::hourly(DEFAULT_PATH.read().join("logs"), "warp-gui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::ERROR.into())
                .from_env_lossy(),
        )
        .init();

    if let Some(title) = opt.title {
        *DEFAULT_WINDOW_NAME.write() = title;
    }

    let tesseract = match Tesseract::from_file(DEFAULT_PATH.read().join(".keystore")) {
        Ok(tess) => tess,
        Err(_) => {
            //doesnt exist so its set
            let tess = Tesseract::default();
            tess.set_file(DEFAULT_PATH.read().join(".keystore"));
            tess.set_autosave();
            tess
        }
    };

    let (account, messaging, storage) = match warp::async_block_in_place_uncheck(initialization(
        DEFAULT_PATH.read().clone(),
        tesseract.clone(),
        opt.experimental_node,
    )) {
        Ok((i, c, s)) => (Account(i.clone()), Messaging(c.clone()), Storage(s.clone())),
        Err(_e) => todo!(),
    };

    let window = WindowBuilder::new()
        .with_title(DEFAULT_WINDOW_NAME.read().clone())
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(950.0, 600.0))
        .with_min_inner_size(LogicalSize::new(330.0, 500.0));
    #[cfg(target_os = "macos")]
    dioxus::desktop::launch_with_props(
        App,
        State {
            tesseract,
            account,
            messaging,
            storage,
        },
        |c| c.with_window(|_| window.with_menu(main_menu)),
    );

    #[cfg(not(target_os = "macos"))]
    dioxus::desktop::launch_with_props(
        App,
        State {
            tesseract,
            account,
            messaging,
            storage,
        },
        |c| c.with_window(|_| window),
    );
}

async fn initialization(
    path: PathBuf,
    tesseract: Tesseract,
    experimental: bool,
) -> Result<(Box<dyn MultiPass>, Box<dyn RayGun>, Box<dyn Constellation>), warp::error::Error> {
    let config = MpIpfsConfig::production(&path, experimental);

    let account = warp_mp_ipfs::ipfs_identity_persistent(config, tesseract, None)
        .await
        .map(|mp| Box::new(mp) as Box<dyn MultiPass>)?;

    let storage = warp_fs_ipfs::IpfsFileSystem::<warp_fs_ipfs::Persistent>::new(
        account.clone(),
        Some(FsIpfsConfig::production(&path)),
    )
    .await
    .map(|ct| Box::new(ct) as Box<dyn Constellation>)?;

    let messaging = warp_rg_ipfs::IpfsMessaging::<Persistent>::new(
        Some(RgIpfsConfig::production(&path)),
        account.clone(),
        Some(storage.clone()),
        None,
    )
    .await
    .map(|rg| Box::new(rg) as Box<dyn RayGun>)?;

    Ok((account, messaging, storage))
}

#[allow(non_snake_case)]
fn App(cx: Scope<State>) -> Element {
    //TODO: Display an error instead of panicing
    std::fs::create_dir_all(DEFAULT_PATH.read().clone()).expect("Error creating directory");
    Config::new_file();

    cx.use_hook(|_| {
        cx.provide_context(cx.props.messaging.clone());
    });
    // Loads the styles for all of our UIKit elements.
    let theme_colors = Theme::load_or_default().rosetta();
    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    let css = include_str!(".styles.css");

    thread::sleep(time::Duration::from_millis(16)); // 60 Hz

    cx.render(rsx!(
        style {
            "{theme_colors}",
            "{css}"
        },
        dioxus_toast::ToastFrame {
            manager: toast,
        }
        AppStyle {},
        span {
            id: "main-wrap",
            ContextMenu {
                parent: String::from("main-wrap"),
                items: cx.render(rsx! {
                    ContextItem {
                        icon: Shape::CodeBracketSquare,
                        text: String::from("View Source"),
                        onpressed: move |_| {
                            let _ = open::that("https://github.com/Satellite-im/Uplink");
                        },
                    }
                })
            },
            Router {
                Route { to: "/", unlock::Unlock { tesseract: cx.props.tesseract.clone() } }
                Route { to: "/loading", loading::Loading { account: cx.props.account.clone() } },
                Route { to: "/auth", auth::Auth { account: cx.props.account.clone() } },
                Route { to: "/main/files", main::files::Files { account: cx.props.account.clone(), storage: cx.props.storage.clone() } },
                Route { to: "/main/friends", main::friends::Friends { account: cx.props.account.clone(), messaging: cx.props.messaging.clone() } },
                Route { to: "/main/settings", main::settings::Settings {
                    account: cx.props.account.clone(),
                    page_to_open: main::settings::sidebar::nav::Route::General,
                }},
                Route { to: "/main/settings/profile", main::settings::Settings {
                    account: cx.props.account.clone(),
                    page_to_open: main::settings::sidebar::nav::Route::Profile,
                }},
                Route { to: "/main", main::Main { account: cx.props.account.clone(), messaging: cx.props.messaging.clone() } },
            }
        }
    ))
}

#[derive(Clone)]
pub struct Messaging(Box<dyn RayGun>);

impl Deref for Messaging {
    type Target = Box<dyn RayGun>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Messaging {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Messaging {
    fn eq(&self, other: &Self) -> bool {
        self.0.id() == other.0.id()
    }
}
