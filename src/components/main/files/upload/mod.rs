use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;

use crate::components::ui_kit::icon_button::IconButton;
use std::future;
use warp::constellation::Constellation;

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
                        onchange: move |e| {
                            // println!("Evt {:?}", e);
                            let _p = e.data.value.clone();
                            let tempVal = file_storage.read().current_directory();

                            use_future(&cx, &cx.props.storage.clone(), |file_storage| async move {
                                let mut _upload_file = match file_storage.write().put("/", &_p).await {
                                    Ok(_) => println!("Ok"),
                                    Err(error) => println!("Error {:?}", error),
                                };
                            });

                            println!("Evt {:?}", tempVal);
                            // let mut _upload_file = match file_storage.write().put("/", &_p) {
                            //     Ok(v) => println!("Evt {:?}", v),
                            //     Err(e) => println!("Evt {:?}", e)
                            // };

                            // let _getFile = file_storage.read().select("/");
                            // upload_file.set(_p);
                            // println!("Evt {:?}", tempVal);
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
