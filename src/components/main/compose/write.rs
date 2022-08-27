use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use crate::components::ui_kit::icon_button::{IconButton, self};

#[derive(Props)]
pub struct Props<'a> {
    submit: EventHandler<'a, ()>,
    // keypress: EventHandler<'a, ()>,
    upload: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Write<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    global_css! ("
        .write {
            display: inline-flex;
            flex-direction: row;
        }
        .button-secondary {
            background: none;
            svg {
                stroke: var(--theme-primary);
            }
        }
    ");
    
    cx.render(rsx!{
        div {
            class: "write",
            IconButton {
                icon: Shape::Plus,
                onclick: move |_| {
                    let _ = &cx.props.upload.call(());
                },
            },
            textarea {
            
            },
            IconButton {
                icon: Shape::ArrowRight,
                state: icon_button::State::Secondary,
                onclick: move |_| {
                    let _ = &cx.props.submit.call(());
                },
            },
        },
    })
}