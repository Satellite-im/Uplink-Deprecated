use dioxus::prelude::*;
use warp::constellation::directory::Directory;

// use crate::components::main::files::sidebar::usage::{Usage, UsageStats};
use crate::{
    components::main::files::{
        browser::FileBrowser, sidebar::Sidebar, toolbar::Toolbar, upload::Upload,
    },
    components::reusable::nav::Nav,
    STATE,
};

#[cfg(target_os = "windows")]
use crate::DRAG_FILE_EVENT;
#[cfg(target_os = "windows")]
use dioxus::desktop::wry::webview::FileDropEvent;

pub mod browser;
pub mod sidebar;
pub mod toolbar;
pub mod upload;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: crate::Storage,
    messaging: crate::Messaging,
}

#[allow(non_snake_case)]
pub fn Files(cx: Scope<Props>) -> Element {
    let show_new_folder = use_state(&cx, || false);
    let show_upload = use_state(&cx, || false);

    let root_directory = match cx.props.storage.current_directory() {
        Ok(current_directory) => current_directory, 
        Err(error) => {
            log::error!("Not possible to get root directory, error: {:?}", error);
            Directory::default()
        },
    };

    if !root_directory.has_item("main_directory") {
        match root_directory.add_directory(Directory::default()) {
            Ok(_) => {
                root_directory.rename_item("un-named directory", "main_directory").unwrap();   
            },
            Err(error) => println!("{error}"),
        }
    }
    let main_directory = root_directory.get_item("main_directory").unwrap().directory().unwrap_or_default();

    let parent_directory = use_ref(&cx, || main_directory);

    let st = use_atom_ref(&cx, STATE).clone();
    let sidebar_visibility = match st.read().hide_sidebar {
        false => "sidebar-visible",
        true => "sidebar-hidden",
    };

    cx.render(rsx! {
        div {
            id: "files",
            onmouseover: |_| {
                // HACK(Windows): Block upload file if drop it anywhere on screen out
                // TODO(Temp): Temp solution to drag and drop work on Windows
                #[cfg(target_os = "windows")]
                {
                *DRAG_FILE_EVENT.write() = FileDropEvent::Cancelled;
                }
            },
            class: "{sidebar_visibility}",
            Sidebar { account: cx.props.account.clone(), messaging: cx.props.messaging.clone() },
            div {
                id: "content",
                rsx!(
                    div {
                        class: "flex-row top-container",
                        Toolbar {
                            on_new_folder: move |_| {
                                show_new_folder.set(!show_new_folder);
                            },
                            on_show_upload: move |_| {
                                show_upload.set(true);
                            }
                        },
                        Upload {
                            storage: cx.props.storage.clone(),
                            show: **show_upload,
                            on_hide: move |_| show_upload.set(false),
                            parent_directory: parent_directory.clone()
                        },
                    },
                    FileBrowser {
                        account: cx.props.account.clone(),
                        storage: cx.props.storage.clone(),
                        show_new_folder: show_new_folder.clone(),
                        parent_directory: parent_directory.clone(),
                    }
                    span {
                        class: "hidden-on-desktop mobile-nav",
                        Nav {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                        }
                    }
                ),
            },
        }
    })
}
