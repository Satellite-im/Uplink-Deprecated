use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::{
    activity_indicator::ActivityIndicator,
    button::Button,
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use warp::crypto::DID;

use crate::{iutils, state::Actions, Messaging, STATE};
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
fn remove_friend(mut multipass: Account, did: DID) -> Result<(), ()> {
    match multipass.remove_friend(&did) {
        Ok(_) => Ok(()),
        Err(error) => {
            log::debug!("error removing friend: {error}");
            Err(())
        }
    }
}

#[allow(non_snake_case)]
pub fn FriendListTile<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    log::debug!("rendering Friend");

    let mp = cx.props.account.clone();
    let rg = cx.props.messaging.clone();

    let username = cx.props.friend_username.clone();
    let show_skeleton = username.is_empty();

    let profile_picture = iutils::get_pfp_from_did(cx.props.friend.clone(), &mp);
    let state = use_atom_ref(&cx, STATE);

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
                            on_pressed: move |_| {
                                let mut rg = rg.clone();
                                let friend = cx.props.friend.clone();
                                let conversation_response = warp::async_block_in_place_uncheck(
                                    rg.create_conversation(&friend)
                                );
                                let conversation = match conversation_response {
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
                        Button {
                            icon: Shape::XMark,
                            state: ui_kit::button::State::Danger,
                            on_pressed: move |_| {
                                let current_chat_exist = state.read().selected_chat;
                                match current_chat_exist {
                                    Some(uuid) => {
                                        let conversation_id = uuid;
                                        if remove_friend(cx.props.account.clone(), cx.props.friend.clone()).is_ok() {
                                            state.write().dispatch(Actions::HideConversation(conversation_id));
                                            log::info!("successfully remove chat from sidebar");
                                        };
                                    },
                                    None => {
                                        if remove_friend(cx.props.account.clone(), cx.props.friend.clone()).is_ok() {
                                            log::info!("Removed friend, but not chat from sidebar!");
                                        };
                                    }
                                }
                                // todo: remove the conversation?
                            }
                        },
                        Button {
                            text:"Block".to_string(),
                            state: ui_kit::button::State::Secondary,
                            on_pressed: move |_| {
                                 let mut multipass = cx.props.account.clone();
                                 let did_to_block = cx.props.friend.clone();
                                 match multipass.block(&did_to_block) {
                                     Ok(_) => {}
                                     Err(e) => {
                                         log::debug!("faied to block friend {}:{}", &cx.props.friend, e);
                                     }
                                 }
                             }
                         },
                    )}
                }
            }
        }
    })
}
