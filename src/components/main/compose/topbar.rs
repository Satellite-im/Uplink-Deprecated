use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;
use uuid::Uuid;
use warp::{crypto::DID, raygun::Conversation};

use crate::{
    components::ui_kit::{
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    MULTIPASS,
};

#[derive(Props)]
pub struct Props<'a> {
    conversation: Conversation,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    global_css!(
        "
        .topbar {
            height: 40px;
            display: inline-flex;
            flex-direction: row;
            padding: 1rem;

            .pfp {
                height: 40px;
                width: 40px;
                border-radius: 20px;
                background: var(--theme-text-muted);
            }
            .who {
                flex: 1;
                heigth: 40px;
                text-align: left;
                padding: 0 1rem;
                display: inline-flex;
                flex-direction: column;

                .top-row {
                    display: inline-flex;
                    flex-direction: row;
                    h3 {
                        margin: 0;
                        font-size: 13pt;
                    }
                }

                span {
                    font-size: 12px;
                    color: var(--theme-text-darker);
                }
            }
            
        }
    "
    );

    // Load Multipass & Raygun's Atom Ref
    let multipass = use_atom_ref(&cx, MULTIPASS);

    // Read their values from locks
    let mp = multipass.read().clone().unwrap().clone();

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
                    span {
                        "{conversation_id}"
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
