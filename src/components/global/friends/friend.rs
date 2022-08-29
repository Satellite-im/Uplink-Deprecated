use dioxus::prelude::*;
use sir::global_css;

use warp::{crypto::DID};

use crate::{
    components::ui_kit::{skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton}},
    MULTIPASS,
};

#[derive(PartialEq, Props)]
pub struct Props {
    friend: String
}

#[allow(non_snake_case)]
pub fn Friend<'a>(cx: Scope<'a, Props>) -> Element<'a> {
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
        .unwrap_or_else(|| "");

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
            )}
            div {
                class: "who",
                if show_skeleton {rsx!(
                    InlineSkeleton {}
                )} else {rsx!(
                    h3 {
                        "{username}"
                    }
                )}
            }
        }
    })
}
