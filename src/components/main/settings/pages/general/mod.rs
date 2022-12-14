use dioxus::prelude::*;

use crate::{iutils::config::Config, Account};
use ui_kit::{input::*, select::*, switch::Switch};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn General(cx: Scope<Props>) -> Element {
    log::debug!("rendering settings/pages/General");
    let mut config = Config::load_config_or_default();

    cx.render(rsx! {
        div {
            id: "page_general",
            class: "padded",
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Splash Screen"
                    },
                    p {
                        "Disabling the splash screen can sometimes make for a faster startup."
                    }
                },
                div {
                    class: "interactive",
                    Switch {
                        active: config.general.show_splash,
                        on_change: move |_| {
                            config.general.show_splash = !config.general.show_splash;
                            let _ = config.save();
                        }
                    }
                }
            }
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Theme"
                    },
                    p {
                        "Select a theme for Uplink."
                    },
                },
                div {
                    class: "interactive",
                    Select {
                        on_change: move |_| {},
                        value: String::from("dark"),
                        options: vec![
                            SelectOption { value: String::from("dark"), label: String::from("Dark") },
                        ]
                    }
                }
            }
        },
    })
}
