use crate::iutils::get_meta::SiteMeta;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use open;
use ui_kit::button;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    meta: SiteMeta,
}

#[allow(non_snake_case)]
pub fn LinkEmbed(cx: Scope<Props>) -> Element {
    // Log a message to the debug output
    log::debug!("rendering LinkEmbed");
    cx.render(rsx! {
        // Check if the title is empty
        if cx.props.meta.title.is_empty() {
            // If the title is empty, return a span with no text
            rsx! { span {""} }
        } else {
            // If the title is not empty, return a div with two child div elements
            rsx! {
                div {
                    class: "link-embed",
                    div {
                        // The first div element has the class "embed-icon"
                        class: "embed-icon",
                        // It contains an img element with the website's icon
                        img {
                            src: "{cx.props.meta.icon}"
                        },
                        // It also contains an h2 element with the website's title
                        h2 {
                            "{cx.props.meta.title}"
                        }
                    }
                    div {
                        // The second div element has the class "embed-details"
                        class: "embed-details",
                        // It contains a p element with the website's description
                        p {
                            "{cx.props.meta.description}"
                        },
                        // It contains a button with the text "Open Link"
                        button::Button {
                            icon: Shape::ArrowDownOnSquare,
                            text: String::from("Open Link"),
                            // The button has the secondary style
                            state: button::State::Secondary,
                            // When the button is clicked, the on_pressed closure is executed
                            on_pressed: move |_| {
                                // The closure calls the open::that function to open the website
                                let _ = open::that(cx.props.meta.url.clone());
                            }
                        }
                    }
                }
            }
        }
    })
}
