use dioxus::{core::to_owned, prelude::*};
use dioxus_heroicons::outline::Shape;
use ui_kit::{
    activity_indicator::ActivityIndicator,
    button::Button,
    profile_picture::PFP,
    skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
};
use warp::{crypto::DID, raygun::Conversation};

use crate::{
    iutils,
    state::{Actions, ConversationInfo},
    Messaging, STATE,
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
fn remove_friend(mut multipass: Account, did: DID) {
    match multipass.remove_friend(&did) {
        Ok(_) => {}
        Err(_) => {
            log::debug!("error removing friend");
        }
    }
}

pub fn Friend<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    log::debug!("rendering Friend");

    let mp = cx.props.account.clone();
    let mut rg = cx.props.messaging.clone();

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
                                let local_state = state.clone();
                                let mut rg = rg.clone();
                                let friend = cx.props.friend.clone();
                                let conversation_response = warp::async_block_in_place_uncheck(
                                    rg.create_conversation(&friend)
                                );
                                let _conversation = match conversation_response {
                                    Ok(v) => v,
                                    Err(warp::error::Error::ConversationExist { conversation }) => conversation,
                                    Err(e) => {
                                        log::error!("failed to chat with friend {}: {}", &cx.props.friend, e);
                                        return;
                                    }
                                };
                                let conversation_params = Conversation::Default::default();
                                local_state.write().dispatch(Actions::ChatWith(Conversation{
                                    id: conversation_params.id(), 
                                    name: conversation_params.name(), 
                                    conversation_type: conversation_params.conversation_type(), 
                                    recipients: conversation_params.recipients()
                                }));
                                cx.props.on_chat.call(());

                            }
                        },
                        Button {
                            icon: Shape::XMark,
                            state: ui_kit::button::State::Danger,
                            on_pressed: move |_| {
                                let local_state = use_atom_ref(&cx, STATE).clone();
                                let rg = cx.props.messaging.clone();
                                let current_chat_exist = local_state.read().selected_chat.clone();
                                match current_chat_exist {
                                    Some(_) => {
                                        let current_chat = local_state.read().selected_chat.and_then(|x| local_state.read().active_chats.get(&x).cloned());
                                        let current_chat_condition = match current_chat {
                                            Some(c) => c,
                                            None => return,
                                        };

                                        let conversation_id = current_chat_condition.conversation.id();

                                        cx.spawn({
                                            to_owned![rg, conversation_id, local_state];
                                            async move {
                                                match rg.delete(conversation_id, None).await {
                                                    Ok(_) => {
                                                        local_state.write().dispatch(Actions::RemoveConversation(conversation_id));
                                                        log::info!("successfully delete conversation")
                                                    },
                                                    Err(error) => log::error!("error when deleting conversation: {error}"),
                                                };
                                            }
                                        });
                                        remove_friend(cx.props.account.clone(), cx.props.friend.clone());
                                    },
                                    None => {
                                        remove_friend(cx.props.account.clone(), cx.props.friend.clone());
                                    }
                                }
                                // todo: remove the conversation?
                            }
                        }
                    )}
                }
            }
        }
    })
}
