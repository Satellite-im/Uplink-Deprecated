use dioxus::prelude::*;

use crate::components::ui_kit::{folder::{State, Folder}, file::File, new_folder::NewFolder};

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    show_new_folder: bool,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "browser",
            (cx.props.show_new_folder).then(|| rsx!(
                NewFolder {
                    state: State::Primary
                }
            )),
            Folder {
                name: String::from("New Folder"),
                state: State::Secondary,
                children: 3
            },
            Folder {
                name: String::from("Examples"),
                state: State::Secondary,
                children: 12
            },
            Folder {
                name: String::from("Logs"),
                state: State::Secondary,
                children: 3941
            },
            File {
                name: String::from("Hello World"),
                state: State::Secondary,
                kind: String::from("txt"),
                size: 0
            },
            File {
                name: String::from("Cache.zip"),
                state: State::Secondary,
                kind: String::from("archive/zip"),
                size: 1
            }
        },
    })
}
