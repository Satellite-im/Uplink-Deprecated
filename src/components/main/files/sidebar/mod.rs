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
                match *display.current() {
                    FolderDisplay::Open => {
                        cx.render(rsx!(
                            Icon {
                                class: "",
                                icon: Shape::FolderOpen,
                            }))
                    }
                    FolderDisplay::Closed => {
                        cx.render(rsx!(
                            Icon {
                                class: "",
                                icon: Shape::Folder,
                            },
                        ))
                    }
                },
                "{folder_name}"
            }

            // can't use if Let to render conditionally based on FolderDisplay because Rust complains of an if without else.
            // it requires the else to return the same type. so now both arms of the match return an Option
            match *display.current() {
                FolderDisplay::Open => {
                    // need to wrap this in cx.render because cx.render returns an Option
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
    let subdir_1 = Directory {
        name: "Subdir1".into(),
        contents: vec![Box::new(DirItem::File(File { name: "f1".into() }))],
    };
    let directory = Directory {
        name: "Folder 1".into(),
        contents: vec![
            Box::new(DirItem::Directory(subdir_1)),
            Box::new(DirItem::File(File { name: "f2".into() })),
        ],
    };
    cx.render(rsx! {
        // TODO: This should be generated based on actual content later.
        // We will create reusable components for a folder and just pass children as data and render
        // recursively automatically.
        // This is just to work on css
        div {
            id: "sidebar",
            class: "tree noselect",
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

/*
#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        // TODO: This should be generated based on actual content later.
        // We will create reusable components for a folder and just pass children as data and render
        // recursively automatically.
        // This is just to work on css
        div {
            id: "sidebar",
            class: "tree noselect",
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
                    div {
                        class: "row",
                        Icon {
                            class: "closed",
                            icon: Shape::Folder,
                        },
                        Icon {
                            class: "open",
                            icon: Shape::FolderOpen,
                        },
                        "Folder 1"
                    },
                    input {
                        id: "tree_folder_control",
                        "type": "checkbox",
                    },
                    label {
                        class: "tree_folder",
                        div {
                            class: "row",
                            Icon {
                                class: "closed",
                                icon: Shape::Folder,
                            },
                            Icon {
                                class: "open",
                                icon: Shape::FolderOpen,
                            },
                            "SubFolder 1"
                        },
                        input {
                            id: "tree_folder_control",
                            "type": "checkbox",
                        },
                        a {
                            class: "tree_item",
                            div {
                                class: "row",
                                Icon {
                                    icon: Shape::Document,
                                },
                                "Item"
                            },
                        },
                    },
                    label {
                        class: "tree_folder",
                        div {
                            class: "row",
                            Icon {
                                class: "closed",
                                icon: Shape::Folder,
                            },
                            Icon {
                                class: "open",
                                icon: Shape::FolderOpen,
                            },
                            "Subfolder 2",
                        },
                        input {
                            id: "tree_folder_control",
                            "type": "checkbox",
                        },
                        label {
                            class: "tree_folder",
                            div {
                                class: "row",
                                Icon {
                                    class: "closed",
                                    icon: Shape::Folder,
                                },
                                Icon {
                                    class: "open",
                                    icon: Shape::FolderOpen,
                                },
                                "Subfolder 1",
                            },
                            input {
                                id: "tree_folder_control",
                                "type": "checkbox",
                            },
                            label {
                                class: "tree_folder",
                                div {
                                    class: "row",
                                    Icon {
                                        class: "closed",
                                        icon: Shape::Folder,
                                    },
                                    Icon {
                                        class: "open",
                                        icon: Shape::FolderOpen,
                                    },
                                    "Subfolder 2",
                                },
                                input {
                                    id: "tree_folder_control",
                                    "type": "checkbox",
                                },
                                a {
                                    class: "tree_item",
                                    div {
                                        class: "row",
                                        Icon {
                                            icon: Shape::Document,
                                        },
                                        "Item"
                                    },
                                },
                            }
                            a {
                                class: "tree_item",
                                div {
                                    class: "row",
                                    Icon {
                                        icon: Shape::Document,
                                    },
                                    "Item"
                                },
                            },
                        }
                        a {
                            class: "tree_item",
                            div {
                                class: "row",
                                Icon {
                                    icon: Shape::Document,
                                },
                                "Item"
                            },
                        },
                    }
                },
                label {
                    class: "tree_folder root",
                    div {
                        class: "row",
                        Icon {
                            class: "closed",
                            icon: Shape::Folder,
                        },
                        Icon {
                            class: "open",
                            icon: Shape::FolderOpen,
                        },
                        "Folder 2"
                    },
                    input {
                        id: "tree_folder_control",
                        "type": "checkbox",
                    },
                    a {
                        class: "tree_item",
                        div {
                            class: "row",
                            Icon {
                                icon: Shape::Document,
                            },
                            "Item"
                        },
                    },
                },
            }
        }
    })
}
*/
