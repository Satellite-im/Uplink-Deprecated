use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use warp::{multipass::MultiPass, sync::RwLock};

pub mod activity_indicator;
pub mod badge;
pub mod button;
pub mod extension_placeholder;
pub mod file;
pub mod folder;
pub mod icon_button;
pub mod icon_input;
pub mod input;
pub mod loader;
pub mod new_folder;
pub mod numeric_indicator;
pub mod photo_picker;
pub mod pin;
pub mod popup;
pub mod profile_picture;
pub mod skeletal_chats;
pub mod skeletons;
pub mod small_extension_placeholder;
pub mod switch;
pub mod textarea;
pub mod tooltip;
pub mod utils;

#[derive(Clone)]
pub struct Account(Arc<RwLock<Box<dyn MultiPass>>>);

impl Deref for Account {
    type Target = Arc<RwLock<Box<dyn MultiPass>>>;
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
        self.0.is_locked() == other.0.is_locked()
    }
}
