use crate::utils::get_meta::SiteMeta;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use open;
use ui_kit::icon_button;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    meta: SiteMeta,
}

#[allow(non_snake_case)]
pub fn LinkEmbed(cx: Scope<Props>) -> Element {
    log::debug!("rendering LinkEmbed");
    cx.render(rsx! {
        if cx.props.meta.title.is_empty() {
            rsx! { span {""} }
        } else {
            rsx! {
                div {
                    class: "link-embed",
                    div {
                        class: "embed-icon",
                        img {
                            src: "{cx.props.meta.icon}"
                        },
                        h2 {
                            "{cx.props.meta.title}"
                        }
                    }
                    div {
                        class: "embed-details",
                        p {
                            "{cx.props.meta.description}"
                        },
                        icon_button::IconButton {
                            icon: Shape::ExternalLink,
                            text: String::from("Open Link"),
                            state: icon_button::State::Secondary,
                            on_pressed: move |_| {
                                let _ = open::that(cx.props.meta.url.clone());
                            }
                        }
                    }
                }
            }
        }
    })
}
