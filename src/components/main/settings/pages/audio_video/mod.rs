use dioxus::prelude::*;

use crate::iutils::config::Config;
use ui_kit::switch::Switch;

#[allow(non_snake_case)]
pub fn AudioVideo(cx: Scope) -> Element {
    log::debug!("rendering settings/pages/AudioVideo");
    let mut config = Config::load_config_or_default();

    cx.render(rsx! {
        div {
            id: "page_audiovideo",
            class: "padded",
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Call Timer"
                    },
                    p {
                        "Display the total time active in a call."
                    }
                },
                div {
                    class: "interactive",
                    Switch {
                        active: config.audiovideo.call_timer,
                        on_change: move |_| {
                            config.audiovideo.call_timer = !config.audiovideo.call_timer;
                            let _ = config.save();
                        }
                    }
                }
            }
        },
    })
}
