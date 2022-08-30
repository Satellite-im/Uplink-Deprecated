use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use warp::{crypto::DID};

use crate::{
    components::ui_kit::{skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton}, icon_button::IconButton},
    MULTIPASS, STATE, state::Actions,
};

#[derive(Props)]
pub struct Props<'a> {
    friend: DID,
    on_chat: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Friend<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let mp = multipass.read().clone().unwrap().clone();

    let friend = cx.props.friend.clone();

    let user = match mp.read().get_identity(friend.clone().into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    let username = user
        .first()
        .map(|i| i.username())
        .unwrap_or_else(|| "".to_string());

    let show_skeleton = username.is_empty();

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
                    font-size: 13px;
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
                                state.write().dispatch(Actions::ChatWith(cx.props.friend.clone())).save();
                                cx.props.on_chat.call(());
                            }
                        }
                    )}
                }
            }
        }
    })
}
