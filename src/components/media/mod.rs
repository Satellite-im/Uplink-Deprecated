use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::button::Button;
use utils::Account;

use crate::{
    components::media::{controls::Controls, media::Media, time::Time},
    iutils::config::Config,
};

pub mod controls;
pub mod media;
pub mod time;

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
}

pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Props)]
pub struct ResizeProps<'a> {
    direction: Direction,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn ResizeContainer<'a>(cx: Scope<'a, ResizeProps<'a>>) -> Element<'a> {
    let class = format!(
        "resize-container {}",
        match cx.props.direction {
            Direction::Horizontal => "horizontal",
            Direction::Vertical => "vertical",
        }
    );

    let script = include_str!("resize.js");

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children,
            div {
                class: "resize-handle",
            }
            script { "{script}" }
        }
    })
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
    let my_identity = mp.get_own_identity().unwrap();
    let username = my_identity.username();
    let names = [username, String::from("Fake User")];
    let config = Config::load_config_or_default();

    let script = include_str!("responsive.js");

    cx.render(rsx! {
        ResizeContainer {
            direction: Direction::Vertical,
            div {
                id: "media-container",
                class: "{class}",
                div {
                    class: "media-view",
                    div {
                        class: "settings-toggle",
                        Button {
                            icon: Shape::Cog,
                            state: ui_kit::button::State::Transparent,
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
                        Button {
                            icon: if **fullscreen { Shape::ArrowsPointingIn } else { Shape::ArrowsPointingOut },
                            state: ui_kit::button::State::Transparent,
                            on_pressed: move |_| fullscreen.set(!fullscreen),
                        }
                    },
                }
                Controls {}
                script { "{script}" }
                config.audiovideo.call_timer.then(|| rsx!{
                    Time {
                        start_time: 0
                    }
                })
            }
        }
    })
}
