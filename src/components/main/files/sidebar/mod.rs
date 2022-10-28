use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use crate::components::main::files::sidebar::usage::{Usage, UsageStats};

pub mod usage;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
}

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
