use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::icon_button::IconButton;
use utils::Account;

use crate::components::media::{controls::Controls, media::Media};

pub mod controls;
pub mod media;

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn MediaContainer(cx: Scope<Props>) -> Element {
    log::debug!("rendering Media Container");
    let fullscreen = use_state(&cx, || false);
    let class = if **fullscreen {
        String::from("fullscreen")
    } else {
        String::from("")
    };

    let mp = cx.props.account.clone();
    let my_identity = mp.read().get_own_identity().unwrap();
    let username = my_identity.username();
    let names = [username, String::from("Fake User")];

    let script = include_str!("responsive.js");

    cx.render(rsx! {
        div {
            id: "media-container",
            class: "{class}",
            div {
                class: "media-view",
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
                    names.iter().map(|name| rsx!(
                        Media {
                            name: name.to_string(),
                            src: "".to_string()
                        }
                    ))
                },
                div {
                    class: "media-toggle",
                    IconButton {
                        icon: if **fullscreen { Shape::Minus } else { Shape::ArrowsExpand },
                        state: ui_kit::icon_button::State::Transparent,
                        on_pressed: move |_| fullscreen.set(!fullscreen),
                    }
                },
            }
            Controls {}
            script { "{script}" }
        }
    })
}
