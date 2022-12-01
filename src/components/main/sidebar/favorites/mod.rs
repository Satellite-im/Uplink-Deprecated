use crate::{
    iutils,
    state::{Actions, ConversationInfo},
    Messaging, LANGUAGE, STATE,
};
use dioxus::prelude::*;
use std::collections::HashMap;
use ui_kit::profile_picture::PFP;
use utils::Account;
use uuid::Uuid;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Favorites(cx: Scope<Props>) -> Element {
    log::debug!("rendering main/sidebar/Favorites");
    let state = use_atom_ref(&cx, STATE);
    let state2 = state.clone();
    let state3 = state2.clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let favString = l.favorites.to_string();

    let all_chats = state.read().active_chats.clone();

    cx.render(rsx!(
        label {
            "{favString}"
        },
        div {
            class: "favorites-container",
            state.read().favorites.clone().iter().filter_map(|chat_id| all_chats.get(chat_id)).cloned().map(|conv_info| {
                let state3 = state3.clone();
                cx.render(rsx!(
                    FavoriteChat {
                        mp: cx.props.account.clone(),
                        conversation_info: conv_info.clone(),
                        on_pressed: move |_| {
                            // this goes to an onclick handler
                            // the onclick event should propagate up to the div with class=popout-mask and close the window
                            if state3.read().selected_chat != Some(conv_info.conversation.id()) {
                                state3.write().dispatch(Actions::ShowChat(conv_info.conversation.id()));
                            }
                        },
                    }
                ))
            })
        },
    ))
}

#[inline_props]
#[allow(non_snake_case)]
pub fn FavoriteChat<'a>(
    cx: Scope,
    mp: Account,
    conversation_info: ConversationInfo,
    on_pressed: EventHandler<'a, Uuid>,
) -> Element<'a> {
    let conversation_id = conversation_info.conversation.id();
    let (did, conversation_name) = iutils::get_username_from_conversation(conversation_info, mp);
    let has_unread = match conversation_info.num_unread_messages > 0 {
        true => "has-unread",
        _ => "",
    };
    let profile_picture = iutils::get_pfp_from_did(did, mp);

    cx.render(rsx! {
        button {
            class: "favorites-item",
            onclick: move |_| on_pressed.call(conversation_id),
            div {
                class: "profile-wrapper",
                div {
                    class: "pfp",
                    PFP {
                        src: profile_picture,
                        size: ui_kit::profile_picture::Size::Normal
                    },
                },
                div {
                    class: "pfs {has_unread}"
                }
            }
            div {
                class: "ellipsis name",
                "{conversation_name}"
            }
        }
    })
}

#[inline_props]
#[allow(non_snake_case)]
pub fn ConversationList<'a>(
    cx: Scope,
    mp: Account,
    all_chats: HashMap<Uuid, ConversationInfo>,
    on_pressed: EventHandler<'a, Uuid>,
) -> Element<'a> {
    log::debug!("rendering ConversationList");
    cx.render(rsx!(
       div {
        class: "add-favorites",
        all_chats.iter().map(|(uuid, conv)| {
            let (did, name) = iutils::get_username_from_conversation(conv, mp);
            let profile_picture = iutils::get_pfp_from_did(did, mp);
            cx.render(rsx!(
                div {
                    class: "to-add",
                    onclick: move |_| on_pressed.call(*uuid),
                    div {
                        class: "pfp",
                        PFP {
                            src: profile_picture,
                            size: ui_kit::profile_picture::Size::Normal
                        },
                    }
                    span {
                        "{name}"
                    }
                }
            ))
        })
       }
    ))
}
