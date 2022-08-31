use dioxus::prelude::*;
use sir::global_css;
use uuid::Uuid;
use warp::raygun::Conversation;


#[derive(PartialEq, Props)]
pub struct Props {
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn TopBar(cx: Scope<Props>) -> Element {
    global_css!(
        "
        .messages {
            background: red;
            
        }
    "
    );

    let conversation_id = cx.props.conversation.id();

    let _show_skeleton = conversation_id == Uuid::default();

    cx.render(rsx! {
        div {
            class: "messages",
        }
    })
}
