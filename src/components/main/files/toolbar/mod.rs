use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;

use crate::components::{reusable::toolbar, ui_kit::icon_button::IconButton};

#[derive(Props)]
pub struct Props<'a> {
    on_new_folder: EventHandler<'a, MouseEvent>,
    on_show_upload: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Toolbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        toolbar::Toolbar {
            controls: rsx! {
                IconButton {
                    icon: Shape::Archive,
                    state: crate::components::ui_kit::icon_button::State::Secondary,
                    on_pressed: move |_| {}
                },
                IconButton {
                    icon: Shape::FolderAdd,
                    state: crate::components::ui_kit::icon_button::State::Secondary,
                    on_pressed: move |e| cx.props.on_new_folder.call(e)
                },
                IconButton {
                    icon: Shape::Upload,
                    on_pressed: move |e| cx.props.on_show_upload.call(e)
                }
            },
            div {}
        },
    })
}
