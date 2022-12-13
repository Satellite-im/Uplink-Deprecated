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
}

#[allow(non_snake_case)]
pub fn NewFolder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name = use_state(&cx, || String::from("New Folder"));
    let show_new_folder = cx.props.show_new_folder.clone();    
    let new_folder_js = include_str!("./new_folder.js");

    cx.render(rsx! {
        div {
            onclick: move |_| {
                cx.props.show_new_folder.set(true);
            },
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
                                if !folder_name.is_empty() {
                                    let file_storage = cx.props.storage.clone();
                                    let new_directory_path = format!("{}", folder_name.clone());
                                    cx.spawn({
                                        to_owned![file_storage, new_directory_path, show_new_folder];
                                        async move {                            
                                            match file_storage.create_directory(&new_directory_path, true).await {
                                                Ok(_) => {
                                                    show_new_folder.set(false);
                                                    log::info!("New directory created. {:?}", new_directory_path);
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
        }
        script { "{new_folder_js}" }
    })


}
