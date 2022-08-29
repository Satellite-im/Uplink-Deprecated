use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use warp::multipass::identity::{FriendRequest, Identity};

use crate::{
    components::ui_kit::icon_button::{self, IconButton},
    MULTIPASS,
};

#[derive(Props)]
pub struct Props<'a> {
    request: FriendRequest,
    on_deny: EventHandler<'a, ()>,
    on_accept: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn FriendRequest<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let mp = multipass.read().clone().unwrap().clone();

    let did = cx.props.request.from();
    let user = match mp.read().get_identity(did.into()) {
        Ok(f) => f,
        Err(_) => vec![],
    };

    let username = "";//user.username();

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
            div {
                class: "pfp"
            },
            div {
                class: "who",
                h3 {
                    "{username}"
                }
            },
            div {
                class: "request-controls",
                div {
                    class: "control-wrap",
                    IconButton {
                        icon: Shape::X,
                        state: icon_button::State::Secondary,
                        on_pressed: move |_| {
                            cx.props.on_accept.call(());
                        }
                    },
                }
                div {
                    class: "control-wrap",
                    IconButton {
                        icon: Shape::Check,
                        state: icon_button::State::Primary,
                        on_pressed: move |_| {
                            cx.props.on_deny.call(());
                        }
                    }
                }
            }
        }
    })
}
