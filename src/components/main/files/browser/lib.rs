use dioxus::prelude::{Scope};
use crate::components::main::files::browser::Props;

pub fn go_back_dirs_with_loop(cx: Scope<Props>, dir_id: uuid::Uuid) {
        let mut file_storage = cx.props.storage.clone();
        loop {
            let current_dir = match file_storage.current_directory() {
                Ok(dir) => dir, 
                _ => break,
            };
            if  current_dir.id() == dir_id {
                cx.needs_update();
                break;
            }
            if let Err(error) = file_storage.go_back() {
                log::error!("Error on go back a directory: {error}");
                break;
            };
        };
    }

