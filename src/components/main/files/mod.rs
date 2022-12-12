use std::{time::Duration, collections::HashSet};

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
    let parent_dir_items = use_ref(&cx,  HashSet::new);


    
    use_future(&cx, (&file_storage, parent_directory, parent_dir_items), 
    |(mut file_storage, parent_directory, parent_dir_items)| 
    async move {
      let parent_dir = parent_directory.with(|dir| dir.clone());
      if parent_dir.name() == "root" {
        loop {
            match file_storage.root_directory().get_item("main_directory") {
                Ok(item) => {
                    match item.get_directory() {
                        Ok(directory) => {
                            parent_directory.with_mut(|dir| *dir = directory.clone());
                            parent_dir_items.with_mut(|_| directory.get_items());
                            log::info!("Main directory was opened. {:?}", directory.name());
                            break;
                        },
                        Err(error) => log::error!("Error opening folder: {error}"),
                    };
                }, 
                Err(error) => {
                    match file_storage.create_directory("main_directory", true).await {
                        Ok(_) => {
                            log::info!("main directory created.")
                        },
                        Err(error) => log::error!("Error creating directory: {error}"),
                    };
                    log::error!("get item from root directory: {error}");}
            };
            tokio::time::sleep(Duration::from_millis(500)).await;
    
           }
      }
    
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
                                show_new_folder.set(true);
                                show_upload.set(false);
                            },
                            on_show_upload: move |_| {
                                show_upload.set(true);
                                show_new_folder.set(false);
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
                        parent_dir_items: parent_dir_items.with(|i| i.clone()),
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
