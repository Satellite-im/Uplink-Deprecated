#![allow(clippy::derive_partial_eq_without_eq)]

use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use crate::{state::Actions, Account, STATE};

#[derive(Eq, PartialEq, Clone)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<DirItem>,
}

#[derive(Eq, PartialEq, Clone)]
pub struct File {
    pub name: String,
}

#[derive(Eq, PartialEq, Clone)]
pub enum DirItem {
    File(File),
    Directory(Directory),
}

#[derive(Eq, PartialEq, Clone)]
pub enum FolderDisplay {
    Open,
    Closed,
}

#[inline_props]
#[allow(non_snake_case)]
pub fn FileElem(cx: Scope, file: File) -> Element {
    let name = file.name.clone();
    let st = use_atom_ref(&cx, STATE).clone();

    cx.render(rsx!(
        a {
            class: "tree-item",
            div {
                class: "row",
                onclick: move |_| {
                    st.write().dispatch(Actions::HideSidebar(true));
                },
                Icon {
                    icon: Shape::Document,
                },
                "{name}"
            },
        },
    ))
}

#[inline_props]
#[allow(non_snake_case)]
pub fn Folder(cx: Scope, dir: Directory) -> Element {
    let display = use_state(&cx, || FolderDisplay::Closed);
    let folder_icon: Shape = match *display.current() {
        FolderDisplay::Open => Shape::FolderOpen,
        FolderDisplay::Closed => Shape::Folder,
    };

    let folder_name = &dir.name;
    cx.render(rsx! {
        div {
            class: "tree-item",
            div {
                class: "row",
                onclick: move |_| {
                    match *display.current() {
                        FolderDisplay::Open => display.set(FolderDisplay::Closed),
                        FolderDisplay::Closed => display.set(FolderDisplay::Open),
                    }
                },
                Icon {
                    class: "",
                    icon: folder_icon,
                },
                "{folder_name}"
            }

            // can't use if Let to render conditionally based on FolderDisplay because Rust complains of an if without else.
            // it requires the else to return the same type. so now both arms of the match return an Option
            match *display.current() {
                FolderDisplay::Open => {
                    // cx.render returns an Option
                    cx.render(rsx!(
                        dir.contents.iter().map(|item| {
                            match item {
                                DirItem::File(f) => cx.render(rsx!(FileElem { file: f.clone() })),
                                DirItem::Directory(d) => cx.render(rsx!(Folder { dir: d.clone() })),
                            }
                        })
                    ))
                }
                FolderDisplay::Closed => None
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<Props>) -> Element {
    // TODO: This should be generated based on actual content later.
    // We will create reusable components for a folder and just pass children as data and render
    // recursively automatically.
    // This is just to work on css
    let subdir_1 = Directory {
        name: "Subdir1".into(),
        contents: vec![DirItem::File(File { name: "f1".into() })],
    };

    let subdir_3 = Directory {
        name: "Subdir3".into(),
        contents: vec![DirItem::File(File { name: "f2".into() })],
    };
    let subdir_2 = Directory {
        name: "Subdir2".into(),
        contents: vec![DirItem::Directory(subdir_3)],
    };
    let directory = Directory {
        name: "Folder 1".into(),
        contents: vec![
            DirItem::Directory(subdir_1),
            DirItem::Directory(subdir_2),
            DirItem::File(File { name: "f3".into() }),
        ],
    };

    // if multiple folders are desired under `Files`, this could render a list of `DirEntry`
    cx.render(rsx! {
        crate::components::reusable::sidebar::Sidebar {
        account: cx.props.account.clone(),
            div {
                class: "label",
                "Files"
            },
            div {
                class: "tree-container",
                Folder {
                    dir: directory
                },
            }
        }
    })
}
