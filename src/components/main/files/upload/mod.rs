use std::path::Path;

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

                                 match file_storage.write().put(&filename, &local_path).await {
                                    Ok(_) => println!("{:?} file uploaded", &filename),
                                    Err(error) => println!("Error to upload file: {:?}, error: {:?}", &filename, error),
                                };
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
