pub mod config;

pub fn remove_writespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}