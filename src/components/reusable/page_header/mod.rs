use dioxus::prelude::*;

use crate::{components::reusable::toolbar, state::Actions, STATE};

use dioxus_heroicons::outline::Shape;
use ui_kit::button::Button;

#[derive(Props)]
pub struct Props<'a> {
    content_start: Element<'a>,
    content_center: Element<'a>,
    content_end: Element<'a>,
    hide_on_desktop: bool,
}

#[allow(non_snake_case)]
pub fn PageHeader<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering PageHeading");

    let header_visibility = match cx.props.hide_on_desktop {
        true => "header-hidden",
        false => "",
    };

    cx.render(rsx! {
        div {
            id: "page-header",
            class: "{header_visibility}",
            toolbar::Toolbar {
                controls: cx.render(rsx! {
                    div {}
                }),
                div {
                    class: "toolbar-content",
                    div {
                        class: "toolbar-start",
                        div {
                            class: "mobile-back-button",
                            Button {
                                icon: Shape::ArrowLeft,
                                state: ui_kit::button::State::Secondary,
                                on_pressed: move |_| {
                                    let state = use_atom_ref(&cx, STATE).clone();
                                    state.write().dispatch(Actions::HideSidebar(false));
                                },
                            },
                        },
                        &cx.props.content_start
                    },
                    div {
                        class: "toolbar-center",
                        &cx.props.content_center
                    },
                    div {
                        class:  "toolbar-end",
                        &cx.props.content_end
                    },
                }
            },
        },
    })
}
