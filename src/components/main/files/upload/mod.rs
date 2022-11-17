use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;

use ui_kit::icon_button::IconButton;

#[derive(Props)]
pub struct Props<'a> {
    show: bool,
    on_hide: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Upload<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        (cx.props.show).then(|| rsx! (
            div {
                id: "upload",
                div {
                    id: "content",
                    input {
                        "type": "file",
                        onchange: move |e| {
                            println!("Evt {:?}", e);
                            let _p = e.data.value.clone();
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
