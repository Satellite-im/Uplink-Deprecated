use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::LANGUAGE;
use ui_kit::button::{self, Button};

#[allow(non_snake_case)]
pub fn Welcome(cx: Scope) -> Element {
    log::debug!("rendering Welcome");
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let router = use_router(&cx).clone();

    cx.render(rsx! {
        div {
            id: "welcome",
            img {
                src: "extra/assets/img/uplink_muted.png"
            },
            p {
                class: "muted",
                "No active chats, wanna make one?"
            },
            Button {
                icon: Shape::Plus,
                text: l.start_one.to_string(),
                state: button::State::Secondary,
                on_pressed: move |_| {
                    router.replace_route("/main/friends", None, None);
                }
            },
        }
    })
}
