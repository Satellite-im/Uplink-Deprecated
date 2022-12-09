use dioxus::{prelude::*, core::to_owned};
use dioxus_elements::KeyCode;
use dioxus_heroicons::{outline::Shape, Icon};
use utils::{Storage, DRAG_FILE_IN_APP_EVENT, DragFileInApp};
use warp::constellation::directory::{Directory};

use crate::context_menu::{ContextItem, ContextMenu};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum State {
    Primary,
    Secondary,
}

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    id: String, 
    name: String,
    state: State,
    size: usize,
    // Maximum amount of items something like HFS Plus could store is 2 billion items
    // Seems to align closet to the 32 bit uint range.
    children: usize,
    storage: Storage,
    parent_directory: UseRef<Directory>,
}

#[allow(non_snake_case)]
pub fn Folder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name = use_state(&cx, || cx.props.name.clone());
    let folder_id = use_state(&cx, || cx.props.id.clone());

    let children = use_state(&cx, || cx.props.children.clone());

    let is_renaming = use_ref(&cx, || false);   

    let parent_directory_ref = cx.props.parent_directory.clone();

    let drag_over_folder = use_ref(&cx, || false);

    cx.render(rsx! {
         div {
            id: "{folder_id}-folder",
            class: "item file",
            onclick: move |_| {
                let file_storage = cx.props.storage.clone();
                let folder_name = cx.props.name.clone();
                let parent_directory = cx.props.parent_directory.clone();
                match file_storage.open_directory(&folder_name) {
                    Ok(directory) => {
                        parent_directory.with_mut(|dir| *dir = directory.clone());
                        log::info!("{folder_name} was opened. {:?}", directory.name());
                    },
                    Err(error) => log::error!("Error opening folder: {error}"),
                };
            },
            ContextMenu {
                parent: format!("{}-folder", folder_id),
                items: cx.render(
                    rsx! {
                            ContextItem {
                                icon: Shape::PencilSquare,
                                onpressed: move |_| {
                                    *is_renaming.write() = true;
                                },
                                text: String::from("Rename")
                            },
                            hr {},
                            ContextItem {
                                onpressed: move |_| {
                                    let folder_name = cx.props.name.clone();
                                    let parent_directory = parent_directory_ref.with(|dir| dir.clone());
                                    match parent_directory.remove_item(&folder_name) {
                                        Ok(_) => {
                                            // TODO: Remove all files inside this folder
                                            log::info!("{folder_name} was deleted.");
                                        },
                                        Err(error) => log::error!("Error deleting folder: {error}"),
                                    }
                                },
                                icon: Shape::Trash,
                                danger: true,
                                text: String::from("Delete")
                            },
                }),
            },
            div {
            class: "folder {class}",
            ondragleave: move |_| {
                *drag_over_folder.write_silent() = false;
            },
            ondragenter: move |_| {
                *drag_over_folder.write_silent() = true;
                let file_storage = cx.props.storage.clone();
                let parent_directory = cx.props.parent_directory.with(|dir| dir.clone());
                cx.spawn({
                    to_owned![parent_directory, file_storage, folder_name, drag_over_folder];
                    async move {
                        loop {
                            let drop_allowed = *drag_over_folder.read();
                            println!("Drop allowed: {:?}", drop_allowed);
                            if drop_allowed == false {
                                break;
                            }
                            let drag_file_event_in_app = get_drag_file_event_in_app();
                            if let Some(file_name) = drag_file_event_in_app.file_name {
                                let root_directory = match file_storage.current_directory() {
                                    Ok(current_directory) => current_directory, 
                                    Err(_) => return,
                                };
                                let current_directory = root_directory.get_item(&folder_name).unwrap().get_directory().unwrap();
                                let file = root_directory.get_item(&file_name).unwrap();
                                match current_directory.add_item(file.clone()) {
                                    Ok(_) => {
                                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                        match parent_directory.remove_item(&file_name) {
                                            Ok(_) => {
                                                *drag_over_folder.write_silent() = false;
                                                // TODO: Remove all files inside this folder
                                                log::info!("file from directory was deleted.");
                                            },
                                            Err(error) => log::error!("Error deleting file from directory: {error}"),
                                        }
                                },
                                    Err(error) => log::error!("Error adding file into directory: {error}"),
                                };
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                        }
                    }
                });
            },

                Icon { icon: Shape::Folder },
                if *is_renaming.read() {
                    rsx! ( input {
                        class: "new_folder_input-{folder_id}",
                        autofocus: "true",
                        placeholder: "New Folder",
                        oninput: move |evt| {
                            folder_name.set(evt.value.to_string());
                        },
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Enter {
                                *is_renaming.write() = false;
                                println!("Create new folder: {}", folder_name.clone());
                            }
                        }
                    })
                } else {
                   rsx!(
                    p { "{folder_name}" },
                   )
                }
                rsx!(label {
                    "{children} item(s)"
                })
            }
        }
    })
}

fn get_drag_file_event_in_app() -> DragFileInApp {
    let drag_file_event_in_app = DRAG_FILE_IN_APP_EVENT.read().clone();
    drag_file_event_in_app
}
