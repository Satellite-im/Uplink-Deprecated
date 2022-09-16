use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use fermi::prelude::*;
use warp::{crypto::DID, error::Error, raygun::Conversation};

use crate::{
    components::ui_kit::{
        activity_indicator::ActivityIndicator,
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    state::Actions,
    Account, Messaging, STATE,
};

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    messaging: Messaging,
    friend: DID,
    on_chat: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Friend<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);

    // Load Multipass & Raygun's Atom Ref
    let multipass = cx.props.account.clone();
    let raygun = cx.props.messaging.clone();

    // Read their values from locks
    let mp = multipass.clone();
    let rg = raygun.clone();

    // Determine our friends DID
    let friend = cx.props.friend.clone();

    let user = match mp.read().get_identity(friend.into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    // std::thread::sleep(std::time::Duration::from_millis(100));

    let username = user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());

    let show_skeleton = username.is_empty();

    cx.render(rsx! {
        div {
            class: "friend",
            if show_skeleton {rsx!(
                PFPSkeleton {}
            )} else {rsx!(
                div {
                    class: "pfp"
                },
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
                    }
                )}
            },
            div {
                class: "request-controls",
                div {
                    class: "control-wrap",
                    if show_skeleton {rsx!(
                        IconButton {
                            icon: Shape::ChatAlt,
                            disabled: true,
                            on_pressed: move |_| {}
                        }
                    )} else {rsx!(
                        IconButton {
                            icon: Shape::ChatAlt,
                            on_pressed: move |_| {
                                let rg = rg.clone();
                                let friend = cx.props.friend.clone();
                                let conversation_response = warp::async_block_in_place_uncheck(
                                    rg.write().create_conversation(&friend)
                                );

                                let conversation = match conversation_response {
                                    Ok(v) => v,
                                    Err(Error::ConversationExist { conversation }) => conversation,
                                    Err(_) => Conversation::default(),
                                };
                                state.write().dispatch(Actions::ChatWith(conversation)).save();
                                cx.props.on_chat.call(());
                            }
                        }
                    )}
                }
            }
        }
    })
}
