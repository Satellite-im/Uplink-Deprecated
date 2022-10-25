use crate::{
    components::ui_kit::{
        activity_indicator::ActivityIndicator,
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    state::ConversationInfo,
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

    // todo: move this into the `impl Conversations` by creating an accessor method
    // use the uuid of the current chat to extract the ConversationInfo from the list
    let opt = &state.read().current_chat.and_then(|conversation_id| {
        // TODO: Make this more dynamic to include multiple PFPs and usernames.
        // Consider code in this todo temporary and only supportive of 2 way convos

        // have to use a vector because of ownership
        let v: Vec<ConversationInfo> = state
            .read()
            .all_chats
            .iter()
            .filter(|x| x.conversation.id() == conversation_id)
            .cloned()
            .collect();

        // expect the vector to have one item
        v.first().cloned()
    });

    match opt {
        Some(conversation_info) => {
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

            let id = conversation_info.conversation.id();

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
                                        "({id})"
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
