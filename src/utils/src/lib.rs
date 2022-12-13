pub mod extensions;
pub mod notifications;
pub mod sounds;

use clap::Parser;
use dioxus::desktop::wry::webview::FileDropEvent;
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use warp::{constellation::Constellation, multipass::MultiPass, sync::RwLock};

#[derive(Debug, Parser)]
#[clap(name = "")]
struct Opt {
    #[clap(long)]
    path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct Account(pub Box<dyn MultiPass>);

impl Deref for Account {
    type Target = Box<dyn MultiPass>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Account {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.0.id() == other.0.id()
    }
}

#[derive(Clone)]
pub struct Storage(pub Box<dyn Constellation>);

impl Deref for Storage {
    type Target = Box<dyn Constellation>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Storage {
    fn eq(&self, other: &Self) -> bool {
        self.0.id() == other.0.id()
    }
}

pub static DEFAULT_PATH: Lazy<RwLock<PathBuf>> = Lazy::new(|| {
    RwLock::new(match Opt::parse().path {
        Some(path) => path,
        _ => dirs::home_dir().unwrap_or_default().join(".warp"),
    })
});

pub static DRAG_FILE_EVENT: Lazy<RwLock<FileDropEvent>> =
    Lazy::new(|| RwLock::new(FileDropEvent::Cancelled));


pub static DRAG_FILE_IN_APP_EVENT: Lazy<RwLock<DragFileInApp>> =
    Lazy::new(|| RwLock::new(DragFileInApp::cancel() ));

#[derive(PartialEq, Clone)]
pub struct DragFileInApp {
    pub file_name: Option<String>,
}

impl DragFileInApp {
    pub fn new_file(file_name: String) -> Self {
        DragFileInApp { file_name: Some(file_name)}
    }

    pub fn cancel() -> Self {
        DragFileInApp { file_name: None}
    }
}