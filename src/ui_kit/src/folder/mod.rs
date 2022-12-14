use dioxus::{prelude::*, core::to_owned, desktop::use_window};
use dioxus_elements::KeyCode;
use dioxus_heroicons::{outline::Shape, Icon};
use utils::{Storage, DRAG_FILE_IN_APP_EVENT, DragFileInApp};
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
    update_current_dir: UseState<()>,
}

#[allow(non_snake_case)]
pub fn Folder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let children = cx.props.children.clone();
    let dir_size = format_folder_size(cx.props.size.clone());


    let folder_name_fmt = format_folder_name_to_show(cx.props.name.clone());

    let folder_name_formatted_state = use_state(&cx, || folder_name_fmt);

    let folder_name_complete_ref = use_ref(&cx, || cx.props.name.clone());

    let folder_id = use_state(&cx, || cx.props.id.clone());
    let drag_over_folder = use_ref(&cx, || false);

    let eval_script = use_window(&cx).clone();

    let file_over_folder_js = include_str!("./file_over_folder.js").replace("folder-id", folder_id);
    let file_leave_folder_js = include_str!("./file_leave_folder.js").replace("folder-id", folder_id);

    let show_edit_name_script = include_str!("./show_edit_name.js").replace("folder_id", &folder_id);

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
                cx.spawn({
                    to_owned![file_storage, folder_name_complete_ref, drag_over_folder, eval_script, folder_id];
                    async move {
                        loop {
                            let drop_allowed = *drag_over_folder.read();
                            if drop_allowed == false {
                                break;
                            }
                            let drag_file_event_in_app = get_drag_file_event_in_app();
                            if let Some(file_name) = drag_file_event_in_app.file_name {
                                let current_directory = file_storage.current_directory().unwrap_or_default();  
                                let folder_name = folder_name_complete_ref.with(|name| name.clone());
                                let directory_target = current_directory.get_item(&folder_name).unwrap().get_directory().unwrap();
                                let file = current_directory.get_item(&file_name).unwrap();
                                match directory_target.add_item(file.clone()) {
                                    Ok(_) => {
                                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                        match current_directory.remove_item(&file_name) {
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
                                    let file_storage = cx.props.storage.clone();
                                    let current_directory = file_storage.current_directory().unwrap_or_default();  
                                    match current_directory.remove_item(&folder_name) {
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
                // let parent_directory = cx.props.parent_directory.clone();
                match file_storage.select(&folder_name) {
                    Ok(_) => {
                        cx.props.update_current_dir.set(());
                        // match file_storage.current_directory() {
                        //     Ok(directory) => {
                        //         println!("Current dir now is {:?}", directory.name());
                        //         parent_directory.with_mut(|dir| *dir = directory.clone());
                        //         log::info!("{folder_name} was opened. {:?}", directory.name());
                        //     },
                        //     Err(error) => println!("Error opening folder: {error}"),
                        // };
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
                                hide_edit_name_element(cx.clone());
                                if !new_folder_name.trim().is_empty() {
                                    cx.spawn({
                                        to_owned![file_storage, old_folder_name, new_folder_name, folder_name_formatted_state, folder_name_complete_ref];
                                        async move {
                                            let new_folder_name = format_args!("{}", new_folder_name.trim()).to_string();
                                          
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
                                    });

                                }

                            }
                        }
                    }
                    label {
                        "{dir_size}"
                        }
                    label {
                        "{children} item(s)"
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

fn format_folder_size(folder_size: usize) -> String {
    if folder_size == 0 {
        return String::from("0 bytes");
    }
    let base_1024: f64 = 1024.0;
    let size_f64: f64 = folder_size as f64;

    let i = (size_f64.log10() / base_1024.log10()).floor();
    let size_formatted = size_f64 / base_1024.powf(i);

    let file_size_suffix = ["bytes", "KB", "MB", "GB", "TB"][i as usize];
    let mut size_formatted_string = format!(
        "{size:.*} {size_suffix}",
        1,
        size = size_formatted,
        size_suffix = file_size_suffix
    );
    if size_formatted_string.contains(".0") {
        size_formatted_string = size_formatted_string.replace(".0", "");
    }
    size_formatted_string
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