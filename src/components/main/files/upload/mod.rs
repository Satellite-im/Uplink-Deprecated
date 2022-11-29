use std::{path::{Path, PathBuf}, io::Cursor};

use dioxus::{core::to_owned, events::{MouseEvent}, prelude::*};
use dioxus_heroicons::outline::Shape;

use futures::StreamExt;
use warp::error::Error;
use mime::*;
use rfd::FileDialog;
use ui_kit::icon_button::IconButton;
use image::io::Reader as ImageReader;

use crate::{Storage, FileDragEvent, DroppedFile};
use crate::DROPPED_FILE;


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

    let observe_drag_event = use_coroutine(&cx, |mut rx: UnboundedReceiver<Action>| {
        async move {
        while let Some(action) = rx.next().await {
            match action {
                Action::Start => {
                        println!("Loop running...");
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                        let dropped_file = DROPPED_FILE.read();
                        if dropped_file.file_drag_event == FileDragEvent::Dropped {
                            let file_path = std::path::Path::new(&dropped_file.local_path).to_path_buf();
                            // upload_file(cx.clone(), file_path);
                        }                            
                    },
                        Action::Stop => {
                        println!("Stopped");
                        },
                    }
                }
            }});

    cx.render(rsx! {
        (cx.props.show).then(|| rsx! (
            div {
                id: "upload",
                div {
                    id: "content",
                    div {
                     
                        div {
                            id: "dropzone",
                            height: "100px",
                            width: "100px",
                            background: "red",
                            oninput: move |evt| println!("input event: {:?}", evt),
                            ondragenter: move |_| {
                                let cx_2 =  cx.clone();
                                observe_drag_event.send(Action::Start);
                            },
                            ondragleave: move |_| {
                                let cx_2 =  cx.clone();

                                observe_drag_event.send(Action::Stop);
                                println!("drag leave");
                            },
                            p { "drag a file here and check your console" }
                        }
                    }
                    input {
                        "type": "file",
                        onclick: move |_| {

                            let file_path = match FileDialog::new().set_directory(".").pick_file() {
                                Some(path) => path,
                                None => return
                            };
                            upload_file(cx.clone(), file_path);
                        }
                    }
                },
                div {
                    id: "close",
                    IconButton {
                        on_pressed: move |e| {
                            cx.props.on_hide.call(e);
                        },
                        state: ui_kit::icon_button::State::Secondary,
                        icon: Shape::XMark
                    }
                }
            }
        ))
    })
}

fn upload_file(cx: Scope<Props>, file_path: PathBuf) {
    let filename = std::path::Path::new(&file_path)
    .file_name()
    .unwrap_or_else(|| std::ffi::OsStr::new(""))
    .to_str()
    .unwrap()
    .to_string();

    let file_storage = cx.props.storage.clone();

    cx.spawn({
        to_owned![file_storage, file_path, filename];
        async move {
            let local_path = Path::new(&file_path).to_string_lossy().to_string();
            let mut filename_to_save = filename.clone();
            let mut count_index_for_duplicate_filename = 1;

            loop {
                match file_storage.put(&filename_to_save, &local_path).await {
                    Ok(_) => {  
                      log::info!("{:?} file uploaded!", &filename_to_save); 

                        match set_thumbnail_if_file_is_image(file_storage, filename_to_save.clone()).await {
                            Ok(success) => log::info!("{:?}", success), 
                            Err(error) => log::error!("Error on update thumbnail: {:?}", error), 
                        }               
                        break;
                    },
                    Err(error) => {
                        match &error {
                            Error::DuplicateName => {

                                let file_name_without_extension = std::path::Path::new(&filename.clone())
                                .with_extension("")
                                .to_str()
                                .unwrap()
                                .to_string();

                                let file_extension = std::path::Path::new(&filename.clone())
                                .extension()
                                .unwrap_or_else(|| std::ffi::OsStr::new(""))
                                .to_str()
                                .unwrap()
                                .to_string();

                                filename_to_save = format!("{} ({}).{}", file_name_without_extension, count_index_for_duplicate_filename, file_extension);
                                log::trace!("Duplicate name, changing file name to {}", &filename_to_save);
                            },
                            _ => {
                                log::error!("Error to upload file: {:?}, error: {:?}", &filename_to_save, error);
                                break;
                            }
                        }
                        count_index_for_duplicate_filename += 1;
                    },
                };
            }
            
            
        }
    });
}


async fn set_thumbnail_if_file_is_image(file_storage: Storage, filename_to_save: String) -> Result<String, Box<dyn std::error::Error>> {
    let item =  file_storage.root_directory().get_item(&filename_to_save)?;
    let parts_of_filename: Vec<&str> = filename_to_save.split('.').collect();

    let file =  file_storage.get_buffer(&filename_to_save).await?;

    // Gurantee that is an image that has been uploaded
    let image = ImageReader::new(Cursor::new(&file)).with_guessed_format()?.decode()?;
    let image_thumbnail = image.thumbnail(70, 70);
  
    // Since files selected are filtered to be jpg, jpeg, png or svg the last branch is not reachable
    let mime = match parts_of_filename.iter().map(|extension| extension.to_lowercase()).last() {
        Some(m) => {
            match m.as_str() {
                "png" => IMAGE_PNG.to_string(),
                "jpg" => IMAGE_JPEG.to_string(),
                "jpeg" => IMAGE_JPEG.to_string(),
                "svg" => IMAGE_SVG.to_string(),
                &_ => "".to_string(),
            }
        },
        None =>  "".to_string(),
    };

    if !file.is_empty() || !mime.is_empty() {
        let prefix = format!("data:{};base64,", mime);
        let base64_image = base64::encode(image_thumbnail.as_bytes());
        let img = prefix + base64_image.as_str();
        item.set_thumbnail(&img);
        Ok(format_args!("{} thumbnail updated with success!", item.name()).to_string())
    } else {
        Err(Box::from(Error::InvalidItem))
    }
}