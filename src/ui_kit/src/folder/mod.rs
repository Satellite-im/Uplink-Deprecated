use std::{path::{PathBuf, Path}, ffi::OsStr, io::Cursor};

use dioxus::{prelude::*, core::to_owned, desktop::{use_window, wry::webview::FileDropEvent, DesktopContext}};
use dioxus_elements::KeyCode;
use dioxus_heroicons::{outline::Shape, Icon};
use mime::*;
use image::io::Reader as ImageReader;
use tokio_util::io::ReaderStream;
use futures::StreamExt;
use utils::{Storage, DRAG_FILE_IN_APP_EVENT, DragFileInApp, DRAG_FILE_EVENT};
use warp::{constellation::Progression, error::Error};
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

    let mut children = cx.props.children;
    let dir_size = format_folder_size(cx.props.size);


    let folder_name_fmt = format_folder_name_to_show(cx.props.name.clone());

    let folder_name_formatted_state = use_state(&cx, || folder_name_fmt);

    let folder_name_complete_ref = use_ref(&cx, || cx.props.name.clone());

    let folder_id = use_state(&cx, || cx.props.id.clone());
    let drag_over_folder = use_ref(&cx, || false);

    let eval_script = use_window(&cx).clone();

    let file_over_folder_js = include_str!("./file_over_folder.js").replace("folder-id", folder_id);
    let file_leave_folder_js = include_str!("./file_leave_folder.js").replace("folder-id", folder_id);

    let show_edit_name_script = include_str!("./show_edit_name.js").replace("folder_id", folder_id);

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
                            if !drop_allowed {
                                break;
                            }
                            let drag_file_event_in_app = get_drag_file_event_in_app();
                            let drag_file_out_app = get_drag_file_event_out_app();

                            if let FileDropEvent::Dropped(files_local_path) = drag_file_out_app {
                                *drag_over_folder.write_silent() = false;
                                for file_path in &files_local_path {
                                    upload_file(
                                        file_storage.clone(),
                                        file_path.clone(),
                                        eval_script.clone(),
                                        folder_name_complete_ref.read().clone(),
                                    )
                                    .await;
                                    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                                    log::info!("{} file uploaded!", file_path.to_string_lossy());
                                }
                                break;
                            }

                            if let Some(file_name) = drag_file_event_in_app.file_name {
                                let current_directory = file_storage.current_directory().unwrap_or_default();  
                                let folder_name = folder_name_complete_ref.with(|name| name.clone());
                               let directory_target = match current_directory.get_item(&folder_name).and_then(|item| item.get_directory()) {
                                    Ok(dir) => dir,
                                    _ => return
                              };
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
                match file_storage.select(folder_name) {
                    Ok(_) => cx.props.update_current_dir.set(()),
                    Err(error) => log::error!("Error selecting new current directory folder: {error}"),
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
                                hide_edit_name_element(cx);
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

fn get_drag_file_event_out_app() -> FileDropEvent {
    let drag_file_event_out_app = DRAG_FILE_EVENT.read().clone();
    drag_file_event_out_app
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

async fn upload_file(file_storage: Storage, file_path: PathBuf, eval_script: DesktopContext, folder_name: String) {
    let mut filename = match file_path
        .file_name()
        .map(|file| file.to_string_lossy().to_string())
    {
        Some(file) => file,
        None => return,
    };

    let local_path = Path::new(&file_path).to_string_lossy().to_string();
    let mut count_index_for_duplicate_filename = 1;
    let mut file_storage = file_storage.clone();
    let original = filename.clone();
    let file = PathBuf::from(&original);
    
    match file_storage.select(&folder_name) {
        Ok(_) => (),
        Err(error) => log::error!("Error selecting new current directory folder: {error}"),
    };
    let current_directory = match file_storage.current_directory() {
        Ok(current_dir) => current_dir, 
        _ => return
    };
    loop {
        if !current_directory.has_item(&filename) {
            break;
        }
        let file_extension = file.extension().and_then(OsStr::to_str).map(str::to_string);
        let file_stem = file.file_stem().and_then(OsStr::to_str).map(str::to_string);

        filename = match (file_stem, file_extension) {
            (Some(file_stem), Some(file_extension)) => {
                format!("{file_stem} ({count_index_for_duplicate_filename}).{file_extension}")
            }
            _ => format!("{original} ({count_index_for_duplicate_filename})"),
        };

        log::info!("Duplicate name, changing file name to {}", &filename);
        count_index_for_duplicate_filename += 1;
    }

    let tokio_file = match tokio::fs::File::open(&local_path).await {
        Ok(file) => file,
        Err(error) => {
            log::error!("Error on get tokio file, cancelling upload action, error: {error}");
            return;
        }
    };

    let total_size_for_stream = match tokio_file.metadata().await {
        Ok(data) => Some(data.len() as usize),
        Err(error) => {
            log::error!("Error getting metadata: {:?}", error);
            None
        }
    };

    let file_stream = ReaderStream::new(tokio_file)
        .filter_map(|x| async { x.ok() })
        .map(|x| x.into());

    match file_storage
        .put_stream(&filename, total_size_for_stream, file_stream.boxed())
        .await
    {
        Ok(mut upload_progress) => {
            while let Some(upload_progress) = upload_progress.next().await {
                match upload_progress {
                    Progression::CurrentProgress {
                        name,
                        current,
                        total,
                    } => {
                        log::info!("Written {} MB for {name}", current / 1024 / 1024);
                        if let Some(total) = total {
                            let mut selector_without_percentage =
                                "document.getElementById('dropzone').value = '".to_owned();

                            let percentage =
                                ((((current as f64) / (total as f64)) * 100.) as usize).to_string();
                            selector_without_percentage.push_str(&percentage);

                            let ending_string = "% uploaded'";
                            selector_without_percentage.push_str(ending_string);

                            eval_script.eval(&selector_without_percentage);

                            log::info!(
                                "{}% completed",
                                (((current as f64) / (total as f64)) * 100.) as usize
                            )
                        }
                    }
                    Progression::ProgressComplete { name, total } => {
                        log::info!(
                            "{name} has been uploaded with {} MB",
                            total.unwrap_or_default() / 1024 / 1024
                        );
                    }
                    Progression::ProgressFailed {
                        name,
                        last_size,
                        error,
                    } => {
                        log::info!(
                            "{name} failed to upload at {} MB due to: {}",
                            last_size.unwrap_or_default(),
                            error.unwrap_or_default()
                        );
                    }
                }
            }
          
            match set_thumbnail_if_file_is_image(file_storage.clone(), filename.clone()).await {
                Ok(success) => log::info!("{:?}", success), 
                Err(error) => log::error!("Error on update thumbnail: {:?}", error), 
            }  

            if let Err(error) = file_storage.go_back() {
                log::error!("Error on go back a directory: {error}");
            };
            
            log::info!("{:?} file uploaded!", &filename);
        }, 
        Err(error) => log::error!("Error when upload file: {:?}", error)
        
    }


    
}

async fn set_thumbnail_if_file_is_image(
    file_storage: Storage,
    filename_to_save: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let item = file_storage.current_directory()?.get_item(&filename_to_save)?;
    let parts_of_filename: Vec<&str> = filename_to_save.split('.').collect();

    let file = file_storage.get_buffer(&filename_to_save).await?;

    // Guarantee that is an image that has been uploaded
    ImageReader::new(Cursor::new(&file))
        .with_guessed_format()?
        .decode()?;

    // Since files selected are filtered to be jpg, jpeg, png or svg the last branch is not reachable
    let mime = match parts_of_filename
        .iter()
        .map(|extension| extension.to_lowercase())
        .last()
    {
        Some(m) => match m.as_str() {
            "png" => IMAGE_PNG.to_string(),
            "jpg" => IMAGE_JPEG.to_string(),
            "jpeg" => IMAGE_JPEG.to_string(),
            "svg" => IMAGE_SVG.to_string(),
            &_ => "".to_string(),
        },
        None => "".to_string(),
    };

    if !file.is_empty() || !mime.is_empty() {
        let prefix = format!("data:{};base64,", mime);
        let base64_image = base64::encode(&file);
        let img = prefix + base64_image.as_str();
        item.set_thumbnail(&img);
        Ok(format_args!("{} thumbnail updated with success!", item.name()).to_string())
    } else {
        Err(Box::from(Error::InvalidItem))
    }
}
