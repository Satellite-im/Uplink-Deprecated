pub mod config;
pub mod notifications;
pub mod sounds;

pub fn remove_writespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
