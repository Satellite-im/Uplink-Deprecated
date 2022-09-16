use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use uuid::Uuid;
use warp::{crypto::DID, raygun::Conversation};
use fermi::prelude::*;
use crate::{
    components::ui_kit::{
        activity_indicator::ActivityIndicator,
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    }, Account,
};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    conversation: Conversation,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    // Load Multipass & Raygun's Atom Ref
    let multipass = cx.props.account.clone();

    // Read their values from locks
    let mp = multipass.clone();

    let conversation_id = cx.props.conversation.id();

    // TODO: Make this more dynamic to include multiple PFPs and usernames.
    // Consider code in this todo temporary and only supportive of 2 way convos
    let display_did = match cx.props.conversation.recipients().last() {
        Some(d) => d.clone(),
        None => DID::default(),
    };
    let display_user = match mp.read().get_identity(display_did.clone().into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };
    let display_username = display_user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());
    // TODO-END

    let show_skeleton = conversation_id == Uuid::default();

    cx.render(rsx! {
        div {
            class: "topbar",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                div {
                    class: "pfp"
                },
            )},
            div {
                class: "who",
                div {
                    class: "top-row",
                    if show_skeleton{rsx!(
                        InlineSkeleton {}
                    )} else {rsx!(
                        h3 {
                            "{display_username}"
                        }
                    )}
                },
                if show_skeleton{rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    div {
                        class: "user-info-inline",
                        ActivityIndicator {
                            inline: true,
                        },
                        p {
                            class: "did",
                            "({conversation_id})"
                        }
                    }
                )}
            },
            div {
                class: "controls",
                IconButton {
                    icon: Shape::Phone,
                    on_pressed: move |_| {
                        cx.props.on_call.call(());
                    },
                }
            }
        },
    })
}
