use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::icon_button::IconButton;

use crate::components::media::{controls::Controls, media::Media};

pub mod controls;
pub mod media;

#[allow(non_snake_case)]
pub fn MediaContainer(cx: Scope) -> Element {
    log::debug!("rendering Media Container");
    let fullscreen = use_state(&cx, || false);
    let class = if **fullscreen {
        String::from("fullscreen")
    } else {
        String::from("")
    };

    cx.render(rsx! {
        div {
            id: "media-container",
            class: "{class}",
            div {
                class: "media-toggle",
                IconButton {
                    icon: if **fullscreen { Shape::Minus } else { Shape::ArrowsExpand },
                    state: ui_kit::icon_button::State::Transparent,
                    on_pressed: move |_| fullscreen.set(!fullscreen),
                }
            },
            div {
                class: "settings-toggle",
                IconButton {
                    icon: Shape::Cog,
                    state: ui_kit::icon_button::State::Transparent,
                    on_pressed: move |_| {},
                }
            },
            div {
                id: "media-content",
                Media {},
                Media {},
                Media {},
            },
            Controls {}
        }
    })
}
