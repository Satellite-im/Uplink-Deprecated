use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use uuid::Uuid;
use warp::{crypto::DID, error::Error, raygun::Conversation};

use crate::{
    components::ui_kit::{
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    state::Actions,
    MULTIPASS, RAYGUN, STATE,
};

#[derive(Props)]
pub struct Props<'a> {
    friend: DID,
    on_chat: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Friend<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);

    // Load Multipass & Raygun's Atom Ref
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let raygun = use_atom_ref(&cx, RAYGUN);

    // Read their values from locks
    let mp = multipass.read().clone().unwrap().clone();
    let rg = raygun.read().clone().unwrap().clone();

    // Determine our friends DID
    let friend = cx.props.friend.clone();

    let user = match mp.read().get_identity(friend.clone().into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    std::thread::sleep(std::time::Duration::from_millis(100));

    let conversation_response =
        warp::async_block_in_place_uncheck(rg.write().create_conversation(&friend));

    let conversation = match conversation_response {
        Ok(v) => v.clone(),
        // TODO: we can't actually add the conversation this way because
        // if the resolve doesn't finish instantly, it will always use the default Uuid.
        // this would be fine if it ever resolved here again, but it doesn't seem to.
        Err(Error::ConversationExist { conversation }) => conversation.clone(),
        Err(_) => Conversation::default(),
    };

    let username = user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());

    let show_skeleton = username.is_empty() || conversation.id() == Uuid::default();

    global_css! {"
        .request {
            display: inline-flex;
            flex-direction: row;
            align-items: center;
            width: 100%;

            .pfp {
                height: 40px;
                width: 40px;
                border-radius: 20px;
                background: var(--theme-text-muted);
            }
            

            .who {
                flex: 1;
                heigth: 40px;
                text-align: left;
                padding: 0 1rem;

                h3 {
                    margin: 0;
                    font-size: 12pt;
                    max-width: 100%;
                }
            }

            .request-controls {
                height: 40px;
                display: inline-flex;

                .control-wrap {
                    margin-left: 1rem;
                }
            }
        }
    "};

    cx.render(rsx! {
        div {
            class: "request",
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
                                state.write().dispatch(Actions::ChatWith(conversation.clone())).save();
                                cx.props.on_chat.call(());
                            }
                        }
                    )}
                }
            }
        }
    })
}
