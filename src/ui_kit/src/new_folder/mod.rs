use dioxus::{prelude::*, core::to_owned};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_html::KeyCode;
use utils::Storage;

use crate::context_menu::{ContextItem, ContextMenu};

use super::folder::State;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    state: State,
    storage: Storage,
}

#[allow(non_snake_case)]
pub fn NewFolder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name = use_state(&cx, || String::from("New Folder"));
    let is_renaming = use_ref(&cx, || true);    


    let folder_id = "12345";

    cx.render(rsx! {

        div {
            id: "{folder_id}-folder",
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
                                 
                                },
                                icon: Shape::Trash,
                                danger: true,
                                text: String::from("Delete")
                            },
                }),
            },
            div {
                class: "folder {class}",
                Icon { icon: Shape::Folder },
                if *is_renaming.read() {
                    rsx! ( input {
                        class: "new_folder_input",
                        autofocus: "true",
                        placeholder: "New Folder",
                        onchange: move |evt| {
                            folder_name.set(evt.value.to_string());
                        },
                        
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Enter {
                                *is_renaming.write() = false;
                                let file_storage = cx.props.storage.clone();
                                let current_directory = match file_storage.current_directory() {
                                    Ok(current_directory) => current_directory, 
                                    Err(error) => {
                                        log::error!("Not possible to get current directory, error: {:?}", error);
                                        return;
                                    },
                                };
                                println!("Folder name: {:?}", folder_name.clone());
                                let new_directory_path = format!("{}", folder_name.clone());
                            
                                cx.spawn({
                                    to_owned![file_storage, new_directory_path];
                                    async move {
                                        println!("New directory path: {:?}", new_directory_path.clone());
                            
                                        match file_storage.create_directory(&new_directory_path, true).await {
                                            Ok(_) => println!(" New directory createad."),
                                            Err(error) => println!("Error creating directory: {error}"),
                                        };
                                    }
                                });
                                println!("Create new folder: {}", folder_name.clone());
                            }
                        }
                    })
                } else {
                   rsx!( p {
                        "{folder_name}"
                    })
                }
    
            }
        }
        
    })
}
