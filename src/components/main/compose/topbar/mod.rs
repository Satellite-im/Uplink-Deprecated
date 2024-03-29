use crate::{
    components::reusable::toolbar,
    iutils::{self, config::Config},
    state::Actions,
    STATE,
};

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::{
    activity_indicator::ActivityIndicator,
    button::Button,
    context_menu::{ContextItem, ContextMenu},
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use utils::Account;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering compose/TopBar");
    let state = use_atom_ref(&cx, STATE);
    let config = Config::load_config_or_default();
    let mut favorites = state.read().favorites.clone();

    // Read their values from locks
    let mp = cx.props.account.clone();

    // todo: move this into the `impl Conversations` by creating an accessor method
    // use the uuid of the current chat to extract the ConversationInfo from the list
    let opt = &state
        .read()
        .selected_chat
        .and_then(|conversation_id| state.read().active_chats.get(&conversation_id).cloned());

    match opt {
        Some(conversation_info) => {
            let (display_did, display_username) =
                iutils::get_username_from_conversation(conversation_info, &mp);
            let profile_picture = iutils::get_pfp_from_did(display_did.clone(), &mp);

            let id = conversation_info.conversation.id();

            let is_favorite = favorites.contains(&id);

            cx.render(rsx! {
                toolbar::Toolbar {
                    controls: cx.render(rsx! {
                        Button {
                            icon: Shape::Heart,
                            state: match is_favorite {
                                true => ui_kit::button::State::Filled,
                                false => ui_kit::button::State::Secondary,
                            },
                            on_pressed: move |_| {
                                match is_favorite {
                                    true => favorites.remove(&id),
                                    false => favorites.insert(id),
                                };
                                state.write().dispatch(Actions::UpdateFavorites(favorites.clone()));
                            },
                        },
                        Button {
                            icon: Shape::Phone,
                            on_pressed: move |_| {
                                cx.props.on_call.call(());
                            },
                        },
                        Button {
                            icon: Shape::VideoCamera,
                            on_pressed: move |_| {
                                cx.props.on_call.call(());
                            },
                        }
                    }),
                    div {
                        class: "mobile-back-button",
                        Button {
                            icon: Shape::ArrowLeft,
                            state: ui_kit::button::State::Secondary,
                            on_pressed: move |_| {
                                state.write().dispatch(Actions::HideSidebar(false));
                            },
                        },
                    },
                    span {
                        id: "pfp-topbar",
                        ContextMenu {
                            parent: String::from("pfp-topbar"),
                            items: cx.render(rsx! {
                                ContextItem {
                                    onpressed: move |_| {},
                                    text: String::from("View Profile"),
                                },
                            })
                        },
                        PFP {
                            src: profile_picture,
                            size: ui_kit::profile_picture::Size::Normal
                        },
                    },
                    div {
                        class: "topbar-user-info",
                        h3 {
                            class: "ellipsis",
                            "{display_username}"
                        }
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
                    }
                }
            })
        }
        None => cx.render(rsx! {
            div {
                class: "topbar-user-info",
                PFPSkeleton {},
                div {
                    InlineSkeleton {},
                    InlineSkeleton {}
                },
                div {
                    class: "topbar-controls",
                    Button {
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
