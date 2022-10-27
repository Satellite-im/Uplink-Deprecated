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
    state::Actions,
    Account, Messaging, LANGUAGE, STATE,
};

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let current_chat = state.read().current_chat;
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let warningMessage = l.prerelease_warning.to_string();

    let blur = state.read().current_chat.is_none();
    let text = use_state(&cx, String::new);
    let show_warning = use_state(&cx, || true);

    // TODO: This is ugly, but we need it for resizing textareas until someone finds a better solution.
    // note that this has a queryselector and click handler specific to this page

    // warning: calling element.style.height='auto' on 'keyup' causes the textarea to randomly resize if you're using shift+enter to make line breaks in the message.
    // this is probably due to releasing the shift key before the enter key.
    // if setting the height is done on 'keydown' then when the enter key is pressed, the event fires before Dioxus clears the TextArea, so the height doesn't change.
    // so a flag has to be set in the 'keydown' event and checked in the 'keyup' event.
    const RESIZE_TEXTAREA_SCRIPT: &str = r#"
    (function addAutoResize() {
        var chat_sent_by_enter = false;

        let element = document.querySelector('.writer-container .resizeable-textarea');
        if (element == null) {
            return;
        }
        let send_button = document.getElementById('send');
        send_button.addEventListener('click', function(event) {
            element.style.height = 'auto';
        });
    
        element.addEventListener('keydown', function(event) {
            if (event.keyCode === 13 && !event.shiftKey) {  
                chat_sent_by_enter = true;
            }
        });

        element.addEventListener('keyup', function(event) {
            if (event.keyCode === 13 && chat_sent_by_enter === true) {
                chat_sent_by_enter = false;
                element.style.height = 'auto';
            }
        });

        element.style.boxSizing = 'border-box';
        var offset = element.offsetHeight - element.clientHeight;
        element.addEventListener('input', function (event) {
            event.target.style.height = 'auto';
            event.target.style.height = event.target.scrollHeight + offset + 'px';
        });
        element.removeAttribute('data-autoresize');
    })()"#;

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
                )),
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

                            if text_as_vec.is_empty() {
                                return;
                            }
                            
                            // clicking the send button is meaningless if there isn't a conversation. 
                            if let Some(id) = current_chat {

                                // mutate the state
                                let cur = state.read().all_chats.get(&id).cloned();
                                if let Some( mut conversation_info) = cur {
                                    conversation_info.last_msg_sent = Some(text_as_vec.iter().take(2).cloned().collect());
                                    state.write().dispatch(Actions::UpdateConversation(conversation_info)).save();
                                }

                                // TODO: We need to wire this message up to display differently
                                // until we confim whether it was successfully sent or failed
                                if let Err(_e) = warp::async_block_in_place_uncheck(rg.send(id, None, text_as_vec)) {
                                    //TODO: Handle error
                                };
                            }
                        },
                        on_upload: move |_| {}
                    }
                },
                script {
                    dangerous_inner_html: "{RESIZE_TEXTAREA_SCRIPT}"
                },
            }
        })
}
