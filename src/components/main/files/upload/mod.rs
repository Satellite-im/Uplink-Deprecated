use std::{
    cmp::Ordering,
    ffi::OsStr,
    io::Cursor,
    path::{Path, PathBuf},
};

use dioxus::{
    core::to_owned,
    desktop::{use_window, wry::webview::FileDropEvent},
    events::MouseEvent,
    prelude::*,
};
use dioxus_heroicons::outline::Shape;

use futures::StreamExt;
use image::io::Reader as ImageReader;
use mime::*;
use rfd::FileDialog;
use ui_kit::button::Button;
use warp::error::Error;

use crate::{Storage, DRAG_FILE_EVENT};

#[derive(Props)]
pub struct Props<'a> {
    storage: crate::Storage,
    show: bool,
    on_hide: EventHandler<'a, MouseEvent>,
}

enum Action {
    Start,
    Stop,
}

#[allow(non_snake_case)]
pub fn Upload<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let file_storage = cx.props.storage.clone();
    let drag_over_dropzone = use_ref(&cx, || false);
    let eval_script = use_window(&cx).clone();
    let file_over_dropzone_js = include_str!("./file_over_dropzone.js");
    let file_leave_dropzone_js = include_str!("./file_leave_dropzone.js");

    let file_being_uploaded_js = "document.getElementById('dropzone').value = 'Uploading...'";

    let upload_file_dropped_routine = use_coroutine(&cx, |mut rx: UnboundedReceiver<Action>| {
        to_owned![
            file_storage,
            drag_over_dropzone,
            eval_script,
            file_leave_dropzone_js,
            file_over_dropzone_js,
            file_being_uploaded_js
        ];
        async move {
            while let Some(action) = rx.next().await {
                match action {
                    Action::Start => {
                        log::info!("File on dropzone");
                        // Time necessary to work on macOS and Linux
                        #[cfg(not(target_os = "windows"))]
                        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                        if *drag_over_dropzone.read() {
                            let drag_file_event = get_drag_file_event();
                            let files_local_path = match drag_file_event.clone() {
                                FileDropEvent::Hovered(files_path)
                                | FileDropEvent::Dropped(files_path) => files_path,
                                _ => Vec::new(),
                            };

                            // TODO(use_eval): Try new solution in the future
                            match files_local_path.len().cmp(&1) {
                                Ordering::Greater => {
                                    let files_to_upload =
                                        format!("{} files to upload!", files_local_path.len());
                                    eval_script.eval(
                                        &file_over_dropzone_js
                                            .replace("file_path", &files_to_upload),
                                    );
                                }
                                Ordering::Equal => {
                                    eval_script.eval(&file_over_dropzone_js.replace(
                                        "file_path",
                                        &files_local_path[0].to_string_lossy(),
                                    ));
                                }
                                _ => {}
                            }

                            if let FileDropEvent::Dropped(files_local_path) = drag_file_event {
                                *drag_over_dropzone.write_silent() = false;
                                // TODO(use_eval): Try new solution in the future
                                eval_script.eval(&file_being_uploaded_js);
                                for file_path in &files_local_path {
                                    upload_file(file_storage.clone(), file_path.clone()).await;
                                    log::info!("{} file uploaded!", file_path.to_string_lossy());
                                }
                                // TODO(use_eval): Try new solution in the future
                                eval_script.eval(&file_leave_dropzone_js);
                            }
                        }
                    }
                    Action::Stop => {
                        eval_script.eval(&file_leave_dropzone_js);
                        log::info!("File not able to upload");
                        // HACK(Temp): Just to improve a little feedback for user on windows
                        // TODO(Temp): Temp solution to drag and drop work on Windows
                        #[cfg(target_os = "windows")]
                        loop {
                            if *drag_over_dropzone.read() {
                                break;
                            }
                            let drag_file_event = get_drag_file_event();
                            match drag_file_event {
                                FileDropEvent::Hovered(files_path) => {
                                    if files_path.len() > 1 {
                                        let files_to_upload = format!(
                                            "Dragging {} files. Drop here to upload them!",
                                            files_path.len()
                                        );
                                        eval_script.eval(
                                            &file_over_dropzone_js
                                                .replace("file_path", &files_to_upload),
                                        );
                                    } else if files_path.len() == 1 {
                                        eval_script.eval(&file_over_dropzone_js.replace(
                                            "file_path",
                                            "Dragging 1 file. Drop here to upload it!",
                                        ));
                                    }
                                }
                                _ => eval_script.eval(&file_leave_dropzone_js),
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        (cx.props.show).then(|| rsx! (
            div {
                id: "upload",
                onmouseover: move |_| {
                    // HACK(Windows): Not allow upload if drop file outside dropzone
                    // TODO(Temp): Temp solution to drag and drop work on Windows
                    #[cfg(target_os = "windows")]
                    if *drag_over_dropzone.read() == false {
                        *DRAG_FILE_EVENT.write() = FileDropEvent::Cancelled;
                    }
                },
                onmouseout:  move |_| {
                    *drag_over_dropzone.write_silent() = false;
                    upload_file_dropped_routine.send(Action::Stop);
                },
                div {
                    id: "content",
                    div {
                        width: "100%",
                        input {
                            "type": "file",
                            prevent_default: "onclick",
                            onclick: move |_| {
                                let files_local_path = match FileDialog::new().set_directory(".").pick_files() {
                                    Some(path) => path,
                                    None => return
                                };
                                let file_storage = cx.props.storage.clone();
                                cx.spawn({
                                    to_owned![file_storage, files_local_path];
                                    async move {
                                        for file_path in &files_local_path {
                                            upload_file(file_storage.clone(), file_path.clone()).await;
                                        }
                                    }
                                });
                            }
                        }
                        hr {
                           class: "hr-between-input-and-dropzone",
                        }
                        input {
                            id: "dropzone",
                            readonly: "true",
                            class: "dropzone",
                            value: "Drop files here to upload",
                            prevent_default: "onmouseover",
                            onmouseover: move |_| {
                                // HACK(Windows): When drop over dropzone, onmouseover is pushed
                                // TODO(Temp): Temp solution to drag and drop work on Windows
                                #[cfg(target_os = "windows")]
                                {
                                let drag_file_event = get_drag_file_event();
                                match drag_file_event {
                                    FileDropEvent::Dropped(_) => {
                                        *drag_over_dropzone.write_silent() = true;
                                        upload_file_dropped_routine.send(Action::Start);
                                    },
                                    _ => {
                                        *drag_over_dropzone.write_silent() = false;
                                        upload_file_dropped_routine.send(Action::Stop);
                                    }
                                }
                                }
                            },
                            onmouseout: move |_| {
                                *drag_over_dropzone.write_silent() = false;
                                upload_file_dropped_routine.send(Action::Stop);
                            },
                            ondragover: move |_| {
                                upload_file_dropped_routine.send(Action::Start);
                            },
                            ondragenter: move |_| {
                                *drag_over_dropzone.write_silent() = true;
                                // TODO(use_eval): Try new solution in the future
                                use_eval(&cx)(&file_over_dropzone_js.replace("file_path", ""));
                            },
                            ondragleave: move |_| {
                                *drag_over_dropzone.write_silent() = false;
                                // TODO(use_eval): Try new solution in the future
                                use_eval(&cx)(&file_leave_dropzone_js);
                                upload_file_dropped_routine.send(Action::Stop);
                            },
                        }
                    }
                },
                div {
                    id: "close",
                    Button {
                        on_pressed: move |e| {
                            cx.props.on_hide.call(e);
                        },
                        state: ui_kit::button::State::Secondary,
                        icon: Shape::XMark
                    }
                }
            }
        ))
    })
}

fn get_drag_file_event() -> FileDropEvent {
    let drag_file_event = DRAG_FILE_EVENT.read().clone();
    drag_file_event
}

async fn upload_file(file_storage: Storage, file_path: PathBuf) {
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
    let current_directory = match file_storage.current_directory() {
        Ok(current_directory) => current_directory,
        Err(error) => {
            log::error!("Not possible to get current directory, error: {:?}", error);
            return;
        }
    };
    let original = filename.clone();

    loop {
        if !current_directory.has_item(&filename) {
            break;
        }
        let file = PathBuf::from(&original);
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

    match file_storage.put(&filename, &local_path).await {
        Ok(_) => {
            log::info!("{:?} file uploaded!", &filename);

            match set_thumbnail_if_file_is_image(file_storage.clone(), filename.clone()).await {
                Ok(success) => log::info!("{:?}", success),
                Err(error) => log::error!("Error on update thumbnail: {:?}", error),
            }
        }
        Err(error) => log::error!("Error to upload file: {:?}, error: {:?}", &filename, error),
    };
}

async fn set_thumbnail_if_file_is_image(
    file_storage: Storage,
    filename_to_save: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let item = file_storage.root_directory().get_item(&filename_to_save)?;
    let parts_of_filename: Vec<&str> = filename_to_save.split('.').collect();

    let file = file_storage.get_buffer(&filename_to_save).await?;

    // Guarantee that is an image that has been uploaded
    let image = ImageReader::new(Cursor::new(&file))
        .with_guessed_format()?
        .decode()?;

    let image_thumbnail = image.thumbnail(70, 70);

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
