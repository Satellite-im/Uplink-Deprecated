use dioxus::prelude::*;

use crate::components::ui_kit::{
    file::File,
    folder::{Folder, State},
};

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "browser",
            Folder {
                name: String::from("New Folder"),
                state: State::Secondary,
                children: 3,
            },
            Folder {
                name: String::from("Examples"),
                state: State::Secondary,
                children: 12,
            },
            Folder {
                name: String::from("Logs"),
                state: State::Secondary,
                children: 3941,
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
        }
    })
}
