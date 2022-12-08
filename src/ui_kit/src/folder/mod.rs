use dioxus::{prelude::*};
use dioxus_elements::KeyCode;
use dioxus_heroicons::{outline::Shape, Icon};
use utils::Storage;
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
    children: u32,
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

    let parent_directory = cx.props.parent_directory.clone();

    cx.render(rsx! {
         div {
            id: "{folder_id}-folder",
            class: "item file",
            ContextMenu {
                parent: format!("{}-folder", folder_id),
                items: cx.render(
                    rsx! {
                            ContextItem {
                                icon: Shape::PencilSquare,
                                onpressed: move |_| {
                            
                                },
                                text: String::from("Rename")
                            },
                            ContextItem {
                                icon: Shape::DocumentArrowDown,
                                onpressed: move |_| {
                                    *is_renaming.write() = true;

                                },
                                text: String::from("Download")
                            },
                            hr {},
                            ContextItem {
                                onpressed: move |_| {
                                    let folder_name = cx.props.name.clone();
                                    match parent_directory.write().remove_item(&folder_name) {
                                        Ok(_) => {
                                            println!("Folder deleted: ");
                                            log::info!("{folder_name} was deleted.");
                                        },
                                        Err(error) => {
                                            println!("error: {:?}", error);
                                            log::error!("Error deleting folder: {error}")},
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
                onclick: move |_| {
                    let file_storage = cx.props.storage.clone();
                    let folder_name = cx.props.name.clone();
                    let parent_directory = cx.props.parent_directory.clone();
                    match file_storage.open_directory(&folder_name) {
                        Ok(directory) => {
                            *parent_directory.write() = directory.clone();
                            println!("{folder_name} was opened. {:?}", directory.name());
                        },
                        Err(error) => {
                            println!("Error opening folder: {error}")},
                    };
                },
                Icon { icon: Shape::Folder },
                if *is_renaming.read() {
                    rsx! ( input {
                        class: "new_folder_input",
                        autofocus: "true",
                        placeholder: "New Folder",
                        oninput: move |evt| {
                            folder_name.set(evt.value.to_string());
                        },
                        
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Enter {
                                *is_renaming.write() = false;
                                // let file_storage = cx.props.storage.clone();
                                // let current_directory = match file_storage.current_directory() {
                                //     Ok(current_directory) => current_directory, 
                                //     Err(error) => {
                                //         log::error!("Not possible to get current directory, error: {:?}", error);
                                //         return;
                                //     },
                                // };
                                // let new_directory_path = format!("{}/{}", current_directory.name(), folder_name.clone());
                            
                                // cx.spawn({
                                //     to_owned![file_storage, new_directory_path];
                                //     async move {
                            
                                //         match file_storage.create_directory(&new_directory_path, true).await {
                                //             Ok(_) => println!(" New directory createad."),
                                //             Err(error) => println!("Error creating direcoty: {error}"),
                                //         };
                                //     }
                                // });
                                println!("Create new folder: {}", folder_name.clone());
                            }
                        }
                    })
                } else {
                   rsx!(     p { "{folder_name}" },
                   label {
                       "{children} item(s)"
                   })
                }
    
            }
        }
    })
}
