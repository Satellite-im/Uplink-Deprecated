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
    show_new_folder: UseState<bool>,
}

#[allow(non_snake_case)]
pub fn NewFolder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name = use_state(&cx, || String::from("New Folder"));
    let is_renaming = use_ref(&cx, || true);    
    let show_new_folder = cx.props.show_new_folder.clone();

    let new_folder_js = include_str!("./new_folder.js");

    cx.render(rsx! {
        script { "{new_folder_js}" }
        div {
            id: "new-folder-id",

            ContextMenu {
                parent: format!("{}-folder", "new-folder-id"),
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
                        id: "new-folder-input",
                        class: "new_folder_input",
                        autofocus: "true",
                        placeholder: "New Folder",
                        onchange: move |evt| {
                            folder_name.set(evt.value.to_string());
                        },
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Enter {
                                *is_renaming.write() = false;
                                show_new_folder.set(false);
                                let file_storage = cx.props.storage.clone();
                                let new_directory_path = format!("{}", folder_name.clone());
                                cx.spawn({
                                    to_owned![file_storage, new_directory_path];
                                    async move {                            
                                        match file_storage.create_directory(&new_directory_path, true).await {
                                            Ok(_) => log::info!(" New directory createad."),
                                            Err(error) => log::error!("Error creating directory: {error}"),
                                        };
                                    }
                                });
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
