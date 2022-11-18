use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use ui_kit::icon_button::IconButton;

use crate::components::reusable::toolbar;

#[derive(Props)]
pub struct Props<'a> {
    on_new_folder: EventHandler<'a, MouseEvent>,
    on_show_upload: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Toolbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        toolbar::Toolbar {
            controls: cx.render(rsx! {
                IconButton {
                    icon: Shape::Archive,
                    state: ui_kit::icon_button::State::Secondary,
                    on_pressed: move |_| {}
                },
                IconButton {
                    icon: Shape::FolderAdd,
                    state: ui_kit::icon_button::State::Secondary,
                    on_pressed: move |e| cx.props.on_new_folder.call(e)
                },
                IconButton {
                    icon: Shape::Upload,
                    on_pressed: move |e| cx.props.on_show_upload.call(e)
                }
            }),
            div {}
        },
    })
}
