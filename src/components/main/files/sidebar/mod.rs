#![allow(clippy::derive_partial_eq_without_eq)]

use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use crate::components::main::files::sidebar::usage::{Usage, UsageStats};

pub mod usage;

#[derive(Eq, PartialEq, Clone)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<Box<DirItem>>,
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
pub fn FileElem(cx: Scope, f: File) -> Element {
    let name = f.name.clone();
    cx.render(rsx!(
        a {
            class: "tree_item",
            div {
                class: "row",
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
            class: "tree_folder",
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
                            // item is referenced by the map
                            // the reference is to a Box, so need another deref
                            // and don't want to move it, so need to borrow
                            match &**item {
                                DirItem::File(f) => cx.render(rsx!(FileElem { f: f.clone() })),
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

#[inline_props]
#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope, _account: crate::Account) -> Element {
    // TODO: This should be generated based on actual content later.
    // We will create reusable components for a folder and just pass children as data and render
    // recursively automatically.
    // This is just to work on css
    let subdir_1 = Directory {
        name: "Subdir1".into(),
        contents: vec![Box::new(DirItem::File(File { name: "f1".into() }))],
    };

    let subdir_3 = Directory {
        name: "Subdir3".into(),
        contents: vec![Box::new(DirItem::File(File { name: "f2".into() }))],
    };
    let subdir_2 = Directory {
        name: "Subdir2".into(),
        contents: vec![Box::new(DirItem::Directory(subdir_3))],
    };
    let directory = Directory {
        name: "Folder 1".into(),
        contents: vec![
            Box::new(DirItem::Directory(subdir_1)),
            Box::new(DirItem::Directory(subdir_2)),
            Box::new(DirItem::File(File { name: "f3".into() })),
        ],
    };

    // if multiple folders are desired under `Files`, this could render a list of `DirEntry`
    cx.render(rsx! {
        div {
            id: "sidebar",
            class: "tree",
            Usage {
                usage: UsageStats {
                    available: 1256,
                    total: 123456,
                    used: 122200,
                    percent_free: 75,
                }
            },
            label {
                class: "m-top-sm",
                "Files"
            },
            div {
                class: "tree_wrapper",
                label {
                    class: "tree_folder root",
                }
                Folder {
                    dir: directory
                },
            }
        }
    })
}
