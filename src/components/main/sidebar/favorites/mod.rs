use crate::{
    components::ui_kit::icon_button::IconButton, state::ConversationInfo, utils, Account,
    Messaging, LANGUAGE, STATE,
};

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use uuid::Uuid;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Favorites(cx: Scope<Props>) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let favString = l.favorites.to_string();
    let newchatdString = l.new_chat.to_string();

    let favorites = state.read().favorites.clone();
    let all_chats = state.read().all_chats.clone();

    cx.render(rsx!(
        label {
            "{favString}"
        },
        div {
            class: "favorites",
            div {
                class: "labeled",
                IconButton {
                    icon: Shape::Plus,
                    on_pressed: move |_| {},
                },
                span {
                    "{newchatdString}"
                }
            },
            favorites.iter().filter_map(|chat_id| all_chats.get(&chat_id)).map(|conv_info| {
                cx.render(rsx!(
                    FavoriteChat {
                        mp: cx.props.account.clone(),
                        conversation_info: conv_info.clone(),
                        on_pressed: move |_| {},
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
    let (_, conversation_name) = utils::get_username_from_conversation(&conversation_info, &mp);
    let color = match conversation_info.num_unread_messages > 0 {
        true => "blue",
        _ => "",
    };
    cx.render(rsx! {
        div {
            class: "favorite-container",
            onclick: move |_| on_pressed.call(conversation_id),
            div {
                class: "pfp"
            },
            div {
                class: "pfs {color}"
            }
            span {
                "{conversation_name}"
            }
        }
    })
}
