// Developer Mode

// Refresh Rate
// - Slug (8 Hz) - 125ms
// - Performance (24 Hz) 41ms
// - Standard (60 Hz) - 16ms
// - Fast (360 Hz) - 3ms
// - Uncapped (Warning, will use all the resources you give it)

// Reset Account

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::crypto::DID;

use crate::{components::ui_kit::{extension_placeholder::ExtensionPlaceholder, switch::Switch, button::{Button, State}}, Account, utils::config::Config};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Developer(cx: Scope<Props>) -> Element {
    let mut config = Config::load_config_or_default();
    let c = config.clone();

    let did = if let Ok(ident) = cx.props.account
        .read()
        .get_own_identity()
    {
        ident.did_key().to_string()
    } else {
        DID::default().to_string()
    };
    cx.render(rsx! {
        div {
            id: "page_developer",
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Developer Mode"
                    },
                    p {
                        "Enabling developer mode adds logging and displays helpful debug information on the UI."
                    }
                },
                div {
                    class: "interactive",
                    Switch {
                        active: config.developer.developer_mode,
                        on_change: move |_| {
                            config.developer.developer_mode = !config.developer.developer_mode;
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
                        "Extract Cache"
                    },
                    p {
                        "Zips and downloads your cache folder for sharing with other developers or migration to another device."
                    }
                },
                div {
                    class: "interactive",
                    Button {
                        icon: Shape::Download,
                        disabled: true,
                        text: String::from("Download"),
                        on_pressed: move |_| {},
                    }
                }
            }
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Reset Cache"
                    },
                    p {
                        "Removes your cacehe and reloads the app, this is useful for testing new accounts quickly."
                    }
                },
                div {
                    class: "interactive",
                    Button {
                        icon: Shape::Trash,
                        state: State::Secondary,
                        text: String::from("Reset"),
                        on_pressed: move |_| {},
                    }
                }
            }
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "DIDKey"
                    },
                    p {
                        class: "selectable",
                        "{did}"
                    }
                },
                div {
                    class: "interactive",
                }
            }
            (c.developer.developer_mode).then(|| rsx! {
                ExtensionPlaceholder {},
            })
        },
    })
}

#[allow(non_snake_case)]
pub fn General(cx: Scope<Props>) -> Element {
    let mut config = Config::load_config_or_default();

    cx.render(rsx! {
        div {
            id: "page_general",
            div {
                class: "item",
                div {
                    class: "description",
                    label {
                        "Splash Screen"
                    },
                    p {
                        "Disabling the splash screen can sometimes make for a faster loading experience."
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
        },
    })
}
