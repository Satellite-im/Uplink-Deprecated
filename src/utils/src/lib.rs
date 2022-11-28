pub mod extensions;
pub mod notifications;
pub mod sounds;

use std::ops::{Deref, DerefMut};

use warp::{constellation::Constellation, multipass::MultiPass};

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