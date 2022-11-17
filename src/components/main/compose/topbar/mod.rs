use crate::{
    components::reusable::toolbar,
    state::Actions,
    utils::{self, config::Config},
    Account, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::{
    activity_indicator::ActivityIndicator,
    icon_button::IconButton,
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};

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
        .current_chat
        .and_then(|conversation_id| state.read().all_chats.get(&conversation_id).cloned());

    match opt {
        Some(conversation_info) => {
            let (display_did, display_username) =
                utils::get_username_from_conversation(conversation_info, &mp);
            let profile_picture = utils::get_pfp_from_did(display_did.clone(), &mp);

            let id = conversation_info.conversation.id();

            let is_favorite = favorites.contains(&id);

            cx.render(rsx! {
                toolbar::Toolbar {
                    controls: cx.render(rsx! {
                        IconButton {
                            icon: Shape::Heart,
                            state: match is_favorite {
                                true => ui_kit::icon_button::State::Filled,
                                false => ui_kit::icon_button::State::Secondary,
                            },
                            on_pressed: move |_| {
                                match is_favorite {
                                    true => favorites.remove(&id),
                                    false => favorites.insert(id),
                                };
                                state.write().dispatch(Actions::UpdateFavorites(favorites.clone()));
                            },
                        },
                        IconButton {
                            icon: Shape::Phone,
                            on_pressed: move |_| {
                                cx.props.on_call.call(());
                            },
                        },
                        IconButton {
                            icon: Shape::VideoCamera,
                            on_pressed: move |_| {
                                cx.props.on_call.call(());
                            },
                        }
                    }),
                    div {
                        class: "mobile-back-button",
                        IconButton {
                            icon: Shape::ArrowLeft,
                            state: ui_kit::icon_button::State::Secondary,
                            on_pressed: move |_| {
                                state.write().dispatch(Actions::HideSidebar(false));
                            },
                        },
                    },
                    PFP {
                        src: profile_picture,
                        size: ui_kit::profile_picture::Size::Normal
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
