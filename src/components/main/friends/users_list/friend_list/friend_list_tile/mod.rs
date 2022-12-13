mod more_menu;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::{
    activity_indicator::ActivityIndicator,
    button::Button,
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use warp::crypto::DID;

use crate::{
    components::main::friends::users_list::friend_list::friend_list_tile::more_menu::MoreMenu,
    iutils, state::Actions, Messaging, STATE,
};
use utils::Account;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    messaging: Messaging,
    friend: DID,
    friend_username: String,
    on_chat: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn FriendListTile<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    log::debug!("rendering Friend");

    let mp = cx.props.account.clone();
    let mut rg = cx.props.messaging.clone();

    let username = cx.props.friend_username.clone();
    let show_skeleton = username.is_empty();

    let profile_picture = iutils::get_pfp_from_did(cx.props.friend.clone(), &mp);
    let state = use_atom_ref(&cx, STATE);

    let friend_id = &cx.props.friend.to_string()[8..];

    let show_more_menu_script = include_str!("./show_more_menu.js").replace("friend_id", friend_id);

    cx.render(rsx! {
        div {
            class: "friend",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                rsx!(PFP {
                    src: profile_picture,
                    size: ui_kit::profile_picture::Size::Normal
                })
            )},
            div {
                class: "who",
                if show_skeleton {rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    h3 {
                        "{username}"
                    },
                    ActivityIndicator {
                        inline: true,
                        remote_did: cx.props.friend.clone(),
                        account: cx.props.account.clone(),
                    }
                )}
            },
            div {
                class: "request-controls",
                div {
                    class: "control-wrap",
                    if show_skeleton {rsx!(
                        Button {
                            icon: Shape::ChatBubbleBottomCenterText,
                            disabled: true,
                            on_pressed: move |_| {}
                        }
                    )} else {rsx!(
                            Button {
                                icon: Shape::ChatBubbleBottomCenterText,
                                // add tooltip text on hover
                                text: "Message".to_string(),
                                hide_text: true,
                                on_pressed: move |_| {
                                    let conversation =
                                    match warp::async_block_in_place_uncheck(rg.create_conversation(&cx.props.friend)) {
                                        Ok(v) => v,
                                        Err(warp::error::Error::ConversationExist { conversation }) => conversation,
                                        Err(e) => {
                                            log::error!("failed to chat with friend {}: {}", &cx.props.friend, e);
                                            return;
                                        }
                                    };
                                    state.write().dispatch(Actions::ChatWith(conversation));
                                    cx.props.on_chat.call(());
                                }
                        },
                        div{
                            div {
                                id:"{friend_id}-more-button",
                                Button {
                                    icon:Shape::EllipsisVertical,
                                    state: ui_kit::button::State::Secondary,
                                    text: "More".to_string(),
                                    hide_text: true,
                                    on_pressed: move |_| {
                                        use_eval(&cx)(&show_more_menu_script);
                                    }
                                },
                            },
                            div {
                                id:"{friend_id}-more-menu",
                                class: "more_menu",
                                display: "none",
                                MoreMenu {
                                    account: cx.props.account.clone(),
                                    friend: cx.props.friend.clone(),
                                    friend_username: cx.props.friend_username.clone(),
                                },
                            },
                        }
                    )}
                }
            }
        }
    })
}
