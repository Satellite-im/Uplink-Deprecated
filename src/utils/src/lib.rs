pub mod extensions;
pub mod notifications;
pub mod sounds;

use std::ops::{Deref, DerefMut};

use warp::multipass::MultiPass;

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
