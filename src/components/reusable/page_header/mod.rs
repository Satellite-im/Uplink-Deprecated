use dioxus::prelude::*;

use crate::{components::reusable::toolbar, state::Actions, STATE};

use dioxus_heroicons::outline::Shape;
use ui_kit::button::Button;

#[derive(Props)]
pub struct Props<'a> {
    // The content to be displayed at the start of the page header
    content_start: Element<'a>,
    // The content to be displayed at the center of the page header
    content_center: Element<'a>,
    // The content to be displayed at the end of the page header
    content_end: Element<'a>,
    // Whether the page header should be hidden on desktop
    hide_on_desktop: bool,
}

#[allow(non_snake_case)]
pub fn PageHeader<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    // Log a debug message
    log::debug!("rendering PageHeading");

    let state = use_atom_ref(&cx, STATE).clone();

    // Determine the visibility class for the page header
    let header_visibility = match cx.props.hide_on_desktop {
        true => "header-hidden",
        false => "",
    };

    // Render the page header
    cx.render(rsx! {
        div {
            id: "page-header",
            // Apply the visibility class
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
                                // When the button is pressed, hide the sidebar
                                on_pressed: move |_| {
                                    state.write().dispatch(Actions::HideSidebar(false));
                                },
                            },
                        },
                        // Render the start content
                        &cx.props.content_start
                    },
                    div {
                        class: "toolbar-center",
                        // Render the center content
                        &cx.props.content_center
                    },
                    div {
                        class:  "toolbar-end",
                        // Render the end content
                        &cx.props.content_end
                    },
                }
            },
        },
    })
}
