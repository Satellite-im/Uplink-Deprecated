use dioxus::{prelude::*, core::to_owned, desktop::use_window};
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
    // children: usize,
    storage: Storage,
    parent_directory: UseRef<Directory>,
}

#[allow(non_snake_case)]
pub fn Folder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name_fmt = format_folder_name_to_show(cx.props.name.clone());

    let folder_name_formatted_state = use_state(&cx, || folder_name_fmt);

    let folder_name_complete_ref = use_ref(&cx, || cx.props.name.clone());

    let folder_id = use_state(&cx, || cx.props.id.clone());
    let parent_directory_ref = cx.props.parent_directory.clone();
    let drag_over_folder = use_ref(&cx, || false);
    let dir_items_len = use_state(&cx, || 0);

    let eval_script = use_window(&cx).clone();

    let file_over_folder_js = include_str!("./file_over_folder.js").replace("folder-id", folder_id);
    let file_leave_folder_js = include_str!("./file_leave_folder.js").replace("folder-id", folder_id);

    let show_edit_name_script = include_str!("./show_edit_name.js").replace("folder_id", &folder_id);

    use_future(&cx, (&cx.props.storage.clone(), dir_items_len),
     |(file_storage, dir_items_len)| async move {
        match file_storage.current_directory() {
            Ok(current_dir) => {
                dir_items_len.set(current_dir.get_items().len());
                log::info!("Update dir {:?} items quantity", current_dir.name());
            }, 
            Err(error) => log::error!("Error get items quantity on a directory: {error}")
        };
    });

    cx.render(rsx! {
         div {
            id: "{folder_id}-folder",
            class: "item file",
            ondragleave: move |_| {
                use_eval(&cx)(&file_leave_folder_js);
                *drag_over_folder.write_silent() = false;
            },
            ondragover: move |_| {
                use_eval(&cx)(&file_over_folder_js);
                *drag_over_folder.write_silent() = true;
                let file_storage = cx.props.storage.clone();
                let parent_directory = &*cx.props.parent_directory.read();
                cx.spawn({
                    to_owned![parent_directory, file_storage, folder_name_complete_ref, drag_over_folder, eval_script, folder_id];
                    async move {
                        loop {
                            let drop_allowed = *drag_over_folder.read();
                            if drop_allowed == false {
                                break;
                            }
                            let drag_file_event_in_app = get_drag_file_event_in_app();
                            if let Some(file_name) = drag_file_event_in_app.file_name {
                                let root_directory = file_storage.root_directory();  
                                let folder_name = folder_name_complete_ref.with(|name| name.clone());
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
                                let file_leave_folder_js = include_str!("./file_leave_folder.js").replace("folder-id", &folder_id);
                                eval_script.eval(&file_leave_folder_js);
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                        }
                    }
                });
            },
            ContextMenu {
                parent: format!("{}-folder", folder_id),
                items: cx.render(
                    rsx! {
                            ContextItem {
                                icon: Shape::PencilSquare,
                                onpressed: move |_| {
                                    //TODO(File): Investigate in a way to replace use_eval in the future
                                    // Use js script to show edit file name element
                                     use_eval(&cx)(&show_edit_name_script);
                                },
                                text: String::from("Rename")
                            },
                            hr {},
                            ContextItem {
                                onpressed: move |_| {
                                    let folder_name = cx.props.name.clone();
                                    let parent_directory = &*parent_directory_ref.read();
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
            onclick: move |_| {
                let mut file_storage = cx.props.storage.clone();
                let folder_name = &*folder_name_complete_ref.read();
                let parent_directory = cx.props.parent_directory.clone();
                match file_storage.select(&folder_name) {
                    Ok(_) => {
                        match file_storage.current_directory() {
                            Ok(directory) => {
                                println!("Current dir now is {:?}", directory.name());
                                parent_directory.with_mut(|dir| *dir = directory.clone());
                                log::info!("{folder_name} was opened. {:?}", directory.name());
                            },
                            Err(error) => println!("Error opening folder: {error}"),
                        };
                    },
                    Err(error) => println!("Error selecting new current directory folder: {error}"),
                };
                
            },         
            Icon { icon: Shape::Folder },
               {
                let val = use_ref(&cx, String::new);
                let complete_folder_name = folder_name_complete_ref.read();
                let folder_id = folder_id.clone();
                    rsx! ( 
                        p { 
                            id: "{folder_id}-name-normal",
                            "{folder_name_formatted_state}" },
                        input {
                        id: "{folder_id}-input",
                        display: "none",
                        class: "new_folder_input",
                        placeholder: "{complete_folder_name}",
                        onchange: move |evt| {
                            val.set(evt.value.to_string());
                        },
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Escape {
                                hide_edit_name_element(cx.clone());
                            }
                            if evt.key_code == KeyCode::Enter {
                                let file_storage = cx.props.storage.clone();
                                let old_folder_name = &*folder_name_complete_ref.read();
                                let new_folder_name = val.read();
                                let parent_directory = cx.props.parent_directory.with(|dir| dir.clone());
                                hide_edit_name_element(cx.clone());
                                if !new_folder_name.trim().is_empty() {
                                    cx.spawn({
                                        to_owned![file_storage, old_folder_name, new_folder_name, folder_name_formatted_state, folder_name_complete_ref, parent_directory];
                                        async move {
                                            let new_folder_name = format_args!("{}", new_folder_name.trim()).to_string();
                                            if let Ok(_) = parent_directory.rename_item(&old_folder_name, &new_folder_name) {
                                                match file_storage.rename(&old_folder_name, &new_folder_name).await {
                                                    Ok(_) => {
                                                    let new_file_name_fmt =
                                                    format_folder_name_to_show(new_folder_name.clone());
                                                        *folder_name_complete_ref.write_silent() = new_folder_name.clone();
                                                        folder_name_formatted_state.set(new_file_name_fmt);
                                                        log::info!("{old_folder_name} renamed to {new_folder_name}");
                                                    },
                                                    Err(error) => log::error!("Error renaming file: {error}"),
                                                };
                                            }
                                        }
                                    });

                                }

                            }
                        }
                    }
                    label {
                        "{dir_items_len} item(s)"
                        }
                )
                } 
            }
        }
    })
}

fn hide_edit_name_element(cx: Scope<Props>) {
    //TODO(File): Investigate in a way to replace use_eval in the future
    // Use js script to hide edit file name element
    let hide_edit_name_script =
        include_str!("./hide_edit_name.js").replace("folder_id", &cx.props.id.clone());
    use_eval(&cx)(&hide_edit_name_script);
}

fn get_drag_file_event_in_app() -> DragFileInApp {
    let drag_file_event_in_app = DRAG_FILE_IN_APP_EVENT.read().clone();
    drag_file_event_in_app
}

fn format_folder_name_to_show(folder_name: String) -> String {
    let mut new_folder_name = folder_name.clone();

    if new_folder_name.len() > 10 {
        new_folder_name = match &new_folder_name.get(0..5) {
            Some(name_sliced) => format!(
                "{}...{}",
                name_sliced,
                &new_folder_name[new_folder_name.len() - 3..].to_string(),
            ),
            None => new_folder_name.clone(),
        };
    }
    new_folder_name
}