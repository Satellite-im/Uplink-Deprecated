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
    // get the first letter of the user's name
    let first_letter = cx.props.name.chars().next().unwrap();
    // uppercase the first letter of the user's name
    let uppercase = first_letter.to_uppercase().collect::<Vec<_>>()[0];
    // get the user's profile picture
    let user_pdp = Some(cx.props.src.clone());

    cx.render(rsx! {
        div {
            class: "media-user",

            div {
                class: "voice-user",
                div {
                    class: "pfp-container",
                    // render the user's profile picture
                    PFP {
                        src: user_pdp.clone(),
                        size: ui_kit::profile_picture::Size::Normal
                    },
                }
                // if the user doesn't have a profile picture, render the first letter of their name instead
                user_pdp.filter(|s| s.is_empty()).map(|_| rsx!(div {
                    class: "placeholder",
                    "{uppercase}"
                }))
            }

            div {
                class: "info-overlay",
                // render the user's name
                div {
                    class: "username ellipsis",
                    "{cx.props.name}"
                }

                // render the microphone icon to indicate that the user is speaking
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
