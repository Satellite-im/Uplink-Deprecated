use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::AppStyle;
use sir::{css, global_css};
use warp::multipass::MultiPass;
use warp::sync::RwLock;
use std::sync::Arc;
use warp::tesseract::Tesseract;

use crate::components::prelude::{unlock, auth};
use crate::components::ui_kit::{self, icon_button, button};
use crate::components::ui_kit::button::Button;
use crate::components::ui_kit::icon_button::IconButton;
use crate::components::ui_kit::tooltip::Tooltip;
use crate::components::ui_kit::switch::Switch;
use crate::components::ui_kit::input::Input;
use crate::components::ui_kit::icon_input::IconInput;
use crate::components::ui_kit::pin::Pin;

pub mod components;
pub mod themes;

#[derive(PartialEq, Props)]
pub struct State {
    locked: bool,
}


static TESSERACT: AtomRef<Tesseract> = |_| Tesseract::default();

fn main() {
    dioxus::desktop::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    global_css!("
        @import url('https://fonts.googleapis.com/css2?family=Poppins&family=Space+Mono&display=swap');
    ");
    

    let container = css!("
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    ");

    let styles = ui_kit::build_style_tag();

    cx.render(rsx! (
        rsx!{
            AppStyle {},
            Router {
                Route { to: "/", unlock::Unlock { pin: String::from("")} }
                Route { to: "/auth", auth::Auth {} },
                Route { 
                    to: "/components", 
                    div {
                        class: "{container}", 
                        div {
                            Button { 
                                text: String::from("Button"),
                            },
                            Button { 
                                text: String::from("Button Icon"),
                                icon: Shape::Chat,
                            },
                            Button { 
                                text: String::from("Button Alt"),
                                state: button::State::Secondary,
                            },
                            Button { 
                                text: String::from("Button Large"),
                                large: true,
                            },
                            IconButton { 
                                icon: Shape::Plus,
                                large: true,
                                onclick: |_| {},
                            },
                            IconButton { 
                                icon: Shape::PhoneOutgoing,
                                large: true,
                                state: icon_button::State::Success,
                                onclick: |_| {},
                            },
                            IconButton { 
                                icon: Shape::PhoneMissedCall,
                                large: true,
                                state: icon_button::State::Danger,
                                onclick: |_| {},
                            },
                            IconButton { 
                                icon: Shape::Microphone
                                large: true,
                                state: icon_button::State::Secondary,
                                onclick: |_| {},
                            },
                            IconButton { 
                                icon: Shape::Plus,
                                onclick: |_| {},
                            },
                            Tooltip {
                                text: String::from("Copied!"),
                                arrow_position: ui_kit::tooltip::ArrowPosition::BottomRight
                            },
                            Tooltip {
                                text: String::from("Copied!"),
                                state: ui_kit::tooltip::State::Secondary,
                                arrow_position: ui_kit::tooltip::ArrowPosition::TopLeft
                            },
                            Switch {
                                active: false
                            },
                            Switch {
                                active: true
                            },
                            Input {
                                placeholder: "Say Something...".to_string(),
                            },
                            IconInput {
                                icon: Shape::Search,
                                placeholder: "Search..".to_string(),
                            },
                            Pin {
                                pin: vec![8],
                                error: true,
                            },
                            Pin {
                                pin: vec![8,3,2],
                                error: true,
                            },
                            Pin {
                                pin: vec![8, 9, 2, 3, 4]
                                error: false,
                            }
                        },
                    },
                }
            }
        }
        style {
            "{styles}"
        }
    ))
}