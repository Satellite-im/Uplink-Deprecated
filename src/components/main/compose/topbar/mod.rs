use crate::{
    components::ui_kit::{
        profile_picture::PFP,
        activity_indicator::ActivityIndicator,
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    utils::{self, config::Config},
    Account, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);
    let config = Config::load_config_or_default();

    // Read their values from locks
    let mp = cx.props.account.clone();

    // todo: move this into the `impl Conversations` by creating an accessor method
    // use the uuid of the current chat to extract the ConversationInfo from the list
    let opt = &state
        .read()
        .current_chat
        .and_then(|conversation_id| state.read().all_chats.get(&conversation_id).cloned());

    match opt {
        Some(conversation_info) => {
            let (display_did, display_username) =
                utils::get_username_from_conversation(conversation_info, &mp);

            let id = conversation_info.conversation.id();

            let identity = mp.read().get_own_identity().unwrap();
            let profile_picture = identity.graphics().profile_picture();


            cx.render(rsx! {
                div {
                    class: "topbar",
                    if profile_picture.is_empty() {
                        rsx! (
                            div {
                                class: "pfp"
                            }  
                        )   
                    } else {
                        rsx!(PFP {
                            src: profile_picture,
                            size: crate::components::ui_kit::profile_picture::Size::Normal
                        })
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
