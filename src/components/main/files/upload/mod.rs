use std::path::Path;

use dioxus::{core::to_owned, events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;

use mime::*;
use rfd::FileDialog;
use ui_kit::icon_button::IconButton;

use warp::constellation::Constellation;

use crate::Storage;

#[derive(Props)]
pub struct Props<'a> {
    storage: crate::Storage,
    show: bool,
    on_hide: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Upload<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let file_storage = cx.props.storage.clone();

    cx.render(rsx! {
        (cx.props.show).then(|| rsx! (
            div {
                id: "upload",
                div {
                    id: "content",
                    input {
                        "type": "file",
                        onclick: move |_| {

                            // TODO(Files): Remove filter to upload other kind of files          
                            let file_path = match FileDialog::new().add_filter("image", &["jpg", "png", "jpeg", "svg"]).set_directory(".").pick_file() {
                                Some(path) => path,
                                None => return
                            };

                            let filename = std::path::Path::new(&file_path)
                            .file_name()
                            .unwrap_or_else(|| std::ffi::OsStr::new(""))
                            .to_str()
                            .unwrap()
                            .to_string();

                            cx.spawn({
                                to_owned![file_storage, file_path, filename];
                                async move {
                                    let local_path = Path::new(&file_path).to_string_lossy().to_string();
                                    let mut filename_to_save = filename.clone();
                                    let mut count_index_for_duplicate_filename = 1;

                                    loop {
                                        match file_storage.put(&filename_to_save, &local_path).await {
                                            Ok(_) => {                                 
                                                update_thumbnail(file_storage, filename_to_save.clone()).await;                 
                                                println!("{:?} file uploaded", &filename_to_save); 
                                                break;
                                            },
                                            Err(error) => {
                                                match &error {
                                                    warp::error::Error::DuplicateName => {

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
                                                        println!("Duplicate name, changing file name to {}", &filename_to_save);
                                                    },
                                                    _ => {
                                                        println!("Error to upload file: {:?}, error: {:?}", &filename_to_save, error);
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
                    }
                },
                div {
                    id: "close",
                    IconButton {
                        on_pressed: move |e| {
                            cx.props.on_hide.call(e);
                        },
                        state: ui_kit::icon_button::State::Secondary,
                        icon: Shape::X
                    }
                }
            }
        ))
    })
}


async fn update_thumbnail(file_storage: Storage, filename_to_save: String) {
    let item =  file_storage.root_directory().get_item(&filename_to_save).unwrap();
    let parts_of_filename: Vec<&str> = filename_to_save.split('.').collect();

    //Since files selected are filtered to be jpg, jpeg, png or svg the last branch is not reachable
    let mime = match parts_of_filename.last() {
        Some(m) => {
            match *m {
                "png" => IMAGE_PNG.to_string(),
                "jpg" => IMAGE_JPEG.to_string(),
                "jpeg" => IMAGE_JPEG.to_string(),
                "svg" => IMAGE_SVG.to_string(),
                &_ => "".to_string(),
            }
        },
        None =>  "".to_string(),
    };
    
    let file =  file_storage.get_buffer(&filename_to_save).await.unwrap_or_default();

    let image = match &file.len() {
        0 => "".to_string(),
        _ => {
            let prefix = format!("data:{};base64,", mime);
            let base64_image = base64::encode(&file);
            let img = prefix + base64_image.as_str();
            img
        }
    };

    item.set_thumbnail(&image);
    println!("Thumbnail setted: {:?}", item.thumbnail());
}