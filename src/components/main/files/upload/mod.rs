use std::{path::Path};

use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;

use crate::components::ui_kit::icon_button::IconButton;
use rfd::FileDialog;

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
                            
                            use_future(&cx, &file_storage, |file_storage| async move {
                                let local_path = Path::new(&file_path).to_string_lossy().to_string();
                                let mut filename_to_save = filename.clone();
                                let mut count_index_for_duplicate_filename = 1;
                                loop {
                                    match file_storage.write().put(&filename_to_save, &local_path).await {
                                        Ok(_) => {println!("{:?} file uploaded", &filename_to_save); 
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
                        state: crate::components::ui_kit::icon_button::State::Secondary,
                        icon: Shape::X
                    }
                }
            }
        ))
    })
}
