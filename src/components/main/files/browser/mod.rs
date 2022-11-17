use dioxus::prelude::*;

use crate::components::ui_kit::{
    file::File,
    folder::{Folder, State},
    new_folder::NewFolder,
};

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: crate::Storage,
    show_new_folder: bool,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {
    let file_storage = cx.props.storage.clone();
    let root_directory = &file_storage.read().root_directory();
    let files = root_directory.get_items();

    cx.render(rsx! {
        div {
            id: "browser",
            (cx.props.show_new_folder).then(|| rsx!(
                NewFolder {
                    state: State::Primary
                }
            )),
            files.iter().map(|file| {
                let file_extension = std::path::Path::new(&file.name())
                .extension()
                .unwrap_or_else(|| std::ffi::OsStr::new(""))
                .to_str()
                .unwrap()
                .to_string();

                rsx!( File {
                    name: file.name(),
                    state: State::Secondary,
                    kind: file_extension,
                    size: file.size(),
                })
            }),
            // Folder {
            //     name: String::from("New Folder"),
            //     state: State::Secondary,
            //     children: 3
            // },
            // Folder {
            //     name: String::from("Examples"),
            //     state: State::Secondary,
            //     children: 12
            // },
            // Folder {
            //     name: String::from("Logs"),
            //     state: State::Secondary,
            //     children: 3941
            // },

            // File {
            //     name: String::from("Hello World"),
            //     state: State::Secondary,
            //     kind: String::from("txt"),
            //     size: 0
            // },
            // File {
            //     name: String::from("Cache.zip"),
            //     state: State::Secondary,
            //     kind: String::from("archive/zip"),
            //     size: 1
            // }
        },
    })
}
