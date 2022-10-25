use crate::{
    components::ui_kit::{
        activity_indicator::ActivityIndicator,
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    utils::config::Config,
    Account, CONVERSATIONS,
};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::multipass::identity::Identity;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, CONVERSATIONS);
    let config = Config::load_config_or_default();

    // Read their values from locks
    let mp = cx.props.account.clone();

    match &state.read().current_chat {
        Some(conversation_info) => {
            // TODO: Make this more dynamic to include multiple PFPs and usernames.
            // Consider code in this todo temporary and only supportive of 2 way convos
            let conversation_id = conversation_info.conversation.id();
            let display_did = conversation_info
                .conversation
                .recipients()
                .last()
                .cloned()
                .unwrap_or_default();

            let display_user = mp
                .read()
                .get_identity(display_did.clone().into())
                .unwrap_or_default();

            let display_username = display_user
                .first()
                .map(Identity::username)
                .unwrap_or_else(String::new);
            // TODO-END

            cx.render(rsx! {
                div {
                    class: "topbar",
                    div {
                        class: "pfp"
                    },
                    div {
                        class: "who",
                        div {
                            class: "top-row",
                            h3 {
                                "{display_username}"
                            }
                        },
                        div {
                            class: "user-info-inline",
                            ActivityIndicator {
                                inline: true,
                                remote_did: display_did.clone(),
                                account: cx.props.account.clone(),
                            },
                            p {
                                class: "did",
                                config.developer.developer_mode.then(|| rsx!(
                                    span {
                                        "({conversation_id})"
                                    }
                                ))
                            }
                        }
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
        None => cx.render(rsx! {
            div {
                class: "topbar",
                PFPSkeleton {},
                div {
                    class: "who",
                    div {
                        class: "top-row",
                        InlineSkeleton {}
                    },
                    InlineSkeleton {}
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
        }),
    }
}
