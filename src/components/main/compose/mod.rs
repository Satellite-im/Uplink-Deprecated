use dioxus::prelude::*;
use sir::global_css;
use warp::crypto::DID;

use crate::{
    components::main::compose::{topbar::TopBar, write::Write},
    STATE,
};

#[derive(PartialEq, Props)]
pub struct Props {
    did: DID,
}

pub mod topbar;
pub mod write;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
    global_css!(
        "
        .compose {
            display: inline-flex;
            flex-direction: column;
            flex: 1;
            position: relative;

            .blurmask {
                -webkit-backdrop-filter: blur(3px);
                background: var(--theme-semi-transparent);
                position: absolute;
                top: 0;
                right: 0;
                bottom: 0;
                left: 0;
                z-index: 90;
            }
            .messages-container {
                flex: 1;
            }
            
            .writer-container {
                width: 100%;
                display: inline-flex;
            }
        }
    "
    );

    let state = use_atom_ref(&cx, STATE);

    let blur = state.read().chat.is_none();
    let text = use_state(&cx, || String::from(""));

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
                        did: cx.props.did.clone(),
                        on_call: move |_| {},
                    }
                )
            },
            div {
                class: "messages-container",
            },
            div {
                class: "writer-container",
                Write {
                    on_submit: move |message| {
                        println!("Send message: {}", message);
                        text.set(String::from(""));
                    },
                    on_upload: move |_| {}
                }
            }
        }
    })
}
