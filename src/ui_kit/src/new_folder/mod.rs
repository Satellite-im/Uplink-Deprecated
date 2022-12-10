use dioxus::{prelude::*, core::to_owned};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_html::KeyCode;
use utils::Storage;
use warp::constellation::directory::Directory;

use super::folder::State;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    state: State,
    storage: Storage,
    show_new_folder: UseState<bool>,
    parent_directory: UseRef<Directory>,
}

#[allow(non_snake_case)]
pub fn NewFolder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name = use_state(&cx, || String::from("New Folder"));
    let show_new_folder = cx.props.show_new_folder.clone();
    let parent_directory_ref = cx.props.parent_directory.clone();
    
    let new_folder_js = include_str!("./new_folder.js");

    cx.render(rsx! {
        div {
            id: "new-folder-id",
            div {
                class: "folder {class}",
                Icon { icon: Shape::Folder }, 
                    input {
                        id: "new-folder-input",
                        class: "new_folder_input",
                        autofocus: "true",
                        placeholder: "New Folder",
                        onchange: move |evt| {
                            folder_name.set(evt.value.to_string());
                        },
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Escape {
                                show_new_folder.set(false);
                            }
                            if evt.key_code == KeyCode::Enter {

                                let file_storage = cx.props.storage.clone();
                                let root_directory = match file_storage.current_directory() {
                                    Ok(current_directory) => current_directory, 
                                    Err(error) => {
                                        log::error!("Not possible to get root directory, error: {:?}", error);
                                        Directory::default()
                                    },
                                };
                                let new_directory_path = format!("{}", folder_name.clone());
                                cx.spawn({
                                    to_owned![file_storage, new_directory_path, root_directory, parent_directory_ref, show_new_folder];
                                    async move {                            
                                        match file_storage.create_directory(&new_directory_path, true).await {
                                            Ok(_) => {
                                                let new_directory = root_directory.get_item(&new_directory_path).unwrap().directory().unwrap_or_default();
                                                let placeholder_dir = Directory::new("placeholder_dir");
                                                // TODO(NewFolder): Refactor later to not be necessary add placeholder dir
                                                // Added placeholder dir, because without it, dioxus lost reference
                                                new_directory.add_directory(placeholder_dir).unwrap();
                                                parent_directory_ref.with_mut(|dir| dir.add_directory(new_directory.clone()).unwrap());
                                                show_new_folder.set(false);
                                                log::info!("New directory created. {:?}", new_directory);
                                            },
                                            Err(error) => log::error!("Error creating directory: {error}"),
                                        };
                                    }
                                });
                            }
                        }
                    }
            }
        }
        script { "{new_folder_js}" }
    })


}
