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
                            // let _p = e.data.value.clone();


                            // println!("Evt {:?}", tempVal);

                            // let _getFile = file_storage.read().select("/");
                            // upload_file.set(_p);
                            // println!("Evt {:?}", tempVal);
                            
                            let file_path = match FileDialog::new().add_filter("image", &["jpg", "png", "jpeg", "svg"]).set_directory(".").pick_file() {
                                Some(path) => path,
                                None => return
                            };
                            let tempVal = &file_storage.read().current_directory();


                            let local_path = Path::new(&file_path).to_string_lossy().to_string();

                            use_future(&cx, &file_storage, |file_storage| async move {
                                let mut _upload_file = match file_storage.write().put("/", &local_path).await {
                                    Ok(_) => println!("Ok"),
                                    Err(error) => println!("Error {:?}", error),
                                };
                            });

                            println!("{:?}", tempVal);
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
