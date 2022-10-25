pub mod messages;
pub mod msg;
pub mod topbar;
pub mod write;

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::raygun::RayGun;

use crate::{
    components::{
        main::compose::{messages::Messages, topbar::TopBar, write::Write},
        ui_kit::icon_button::IconButton,
    },
    Account, Messaging, CONVERSATIONS, LANGUAGE,
};

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
    let state = use_atom_ref(&cx, CONVERSATIONS);
    let ext_conversation_id = state
        .read()
        .current_chat
        .as_ref()
        .map(|c| c.conversation.id());
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let warningMessage = l.prerelease_warning.to_string();

    let blur = state.read().current_chat.is_none();
    let text = use_state(&cx, String::new);
    let show_warning = use_state(&cx, || true);

    // todo: render normally
    cx.render(rsx! {
        div {
            class: "compose",
            if blur {
                rsx!(
                    div {
                        class: "blurmask"
                    }
                )
            } else {
                rsx!(
                    TopBar {
                        account: cx.props.account.clone(),
                        on_call: move |_| {},
                    }
                )
            },
            (**show_warning).then(|| rsx!(
                div {
                    class: "alpha-warning animate__animated animate__slideInDown",
                    "{warningMessage}",
                    IconButton {
                        on_pressed: move |_| {
                            show_warning.set(false);
                        },
                        icon: Shape::Check,
                    }
                },
            ))
            div {
                class: "messages-container",
                div { class: "gradient_mask" },
                Messages {
                    account: cx.props.account.clone(),
                    messaging: cx.props.messaging.clone(),
                }
                div { class: "gradient_mask is_bottom" },
            },
            div {
                class: "writer-container",
                Write {
                    on_submit: move |message: String| {
                        text.set(String::from(""));
                        let mut rg = cx.props.messaging.clone();

                        let text_as_vec = message
                            .split('\n')
                            .filter(|&s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();

                        // clicking the send button is meaningless if there isn't a conversation. 
                        if let Some(id) = ext_conversation_id {
                            // TODO: We need to wire this message up to display differently
                            // until we confim whether it was successfully sent or failed
                            if let Err(_e) = warp::async_block_in_place_uncheck(rg.send(id, None, text_as_vec)) {
                                //TODO: Handle error
                            };
                        }
                    },
                    on_upload: move |_| {}
                }
            }
        }
    })
}
