use dioxus::prelude::*;

use crate::components::reusable::toolbar;
#[derive(Props)]
pub struct Props<'a> {
    content_start: Option<Element<'a>>,
    content_center: Option<Element<'a>>,
    content_end: Option<Element<'a>>,
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
                controls: cx.render(rsx! { Fragment {} }),
                div {
                    class: "toolbar-content",
                    div {
                        class: "toolbar-start",
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
