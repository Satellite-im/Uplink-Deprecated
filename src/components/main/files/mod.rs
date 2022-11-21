use dioxus::prelude::*;

// use crate::components::main::files::sidebar::usage::{Usage, UsageStats};
use crate::{
    components::reusable::nav::Nav,
    main::files::{
        browser::FileBrowser,
        sidebar::usage::{Usage, UsageStats},
        toolbar::Toolbar,
        upload::Upload,
    },
};
pub mod browser;
pub mod sidebar;
pub mod toolbar;
pub mod upload;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: crate::Storage,
}

#[allow(non_snake_case)]
pub fn Files(cx: Scope<Props>) -> Element {
    let show_new_folder = use_state(&cx, || false);
    let show_upload = use_state(&cx, || false);

    cx.render(rsx! {
        div {
            id: "files",
            sidebar::Sidebar { account: cx.props.account.clone() },
            div {
                id: "content",
                rsx!(
                    div {
                        class: "flex-row top-container",
                        div {
                            class: "mobile-usage",
                            span {
                                class: "hidden-on-desktop",
                                Usage {
                                    usage: UsageStats {
                                        available: 1256,
                                        total: 123456,
                                        used: 122200,
                                        percent_free: 61,
                                    }
                                },
                            }
                        },
                        Toolbar {
                            on_new_folder: move |_| {
                                show_new_folder.set(true);
                            },
                            on_show_upload: move |_| {
                                show_upload.set(true);
                            }
                        },
                        Upload {
                            storage: cx.props.storage.clone(),
                            show: **show_upload,
                            on_hide: move |_| show_upload.set(false),
                        },
                    },
                    FileBrowser {
                        account: cx.props.account.clone(),
                        storage: cx.props.storage.clone(),
                        show_new_folder: **show_new_folder
                    }
                    span {
                        class: "hidden-on-desktop mobile-nav",
                        Nav {
                            account: cx.props.account.clone(),
                        }
                    }
                ),
            },
        }
    })
}
