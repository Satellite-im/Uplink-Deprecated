use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use ui_kit::profile_picture::PFP;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    name: String,
    src: String,
}

#[allow(non_snake_case)]
pub fn Media(cx: Scope<Props>) -> Element {
    log::debug!("rendering Media");
    let first_letter = cx.props.name.chars().next().unwrap();
    let uppercase = first_letter.to_uppercase().collect::<Vec<_>>()[0];
    let user_pdp = Some(cx.props.src.clone());

    cx.render(rsx! {
        div {
            class: "media-user",

            div {
                class: "voice-user",
                div {
                    class: "pfp-container",
                    PFP {
                        src: user_pdp.clone(),
                        size: ui_kit::profile_picture::Size::Normal
                    },
                }
                user_pdp.filter(|s| s.is_empty()).map(|_| rsx!(div {
                    class: "placeholder",
                    "{uppercase}"
                }))
            }

            div {
                class: "info-overlay",
                div {
                    class: "username ellipsis",
                    "{cx.props.name}"
                }

                div {
                    class: "indicator",
                    Icon {
                        icon: Shape::Microphone
                    },
                }
            }
        }
    })
}
