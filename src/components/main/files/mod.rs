use std::time::Duration;

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
use utils::DRAG_FILE_EVENT;
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

    let file_storage = cx.props.storage.clone();

    let root_directory = match file_storage.current_directory() {
        Ok(current_directory) => current_directory, 
        Err(error) => {
            log::error!("Not possible to get root directory, error: {:?}", error);
            Directory::default()
        },
    };   

    let parent_directory = use_ref(&cx, || root_directory.clone());

    
    use_future(&cx, (&file_storage, parent_directory, &root_directory), |(mut file_storage, parent_directory, root_directory)| async move {
        if !root_directory.has_item("main_directory") {
            match file_storage.create_directory("main_directory", true).await {
                Ok(_) => {
                    log::info!("main directory created.")
                },
                Err(error) => log::error!("Error creating directory: {error}"),
            };
        };

        if &*parent_directory.read().name() == "root" {
            match file_storage.open_directory("main_directory") {
                Ok(directory) => {
                    parent_directory.with_mut(|dir| *dir = directory.clone());
                    log::info!("Main directory was opened. {:?}", directory.name());
                },
                Err(error) => log::error!("Error opening folder: {error}"),
            };
        }
            tokio::time::sleep(Duration::from_millis(100)).await;
            return;
        });
        
  
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
