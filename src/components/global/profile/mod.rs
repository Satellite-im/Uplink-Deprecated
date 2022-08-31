use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use crate::{
    components::ui_kit::{popup::Popup, button::Button},
};

#[derive(Props)]
pub struct Props<'a> {
    show: bool,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Profile<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    global_css! {"
        .profile {
            display: inline-flex;
            flex-direction: column;
            width: 100%;
            
            .background {
                height: 140px;
                width: 100%;
                background: var(--theme-text-muted);
                margin-bottom: 40px;
                position: relative;
                border-radius: 8px 8px 0 0;

                .profile-photo {
                    width: 80px;
                    height: 80px;
                    position: absolute;
                    bottom: -41px;
                    left: calc(50% - 41px);
                    border-radius: 80px;
                    background: var(--theme-text-muted);
                    border: 2px solid var(--theme-foreground);
                }
            }

            .profile-body {
                width: 100%;

                .meta {
                    width: 100%;
                    display: inline-flex;
                    flex-direction: row;
                    justify-content: space-between;
                }

            }
        }
    "};

    cx.render(rsx! {
        Popup {
            on_dismiss: |_| cx.props.on_hide.call(()),
            hidden: !cx.props.show,
            children: cx.render(rsx!(
                div {
                    class: "profile",
                    div {
                        class: "background",
                        div {
                            class: "profile-photo",
                        }
                    },
                    div {
                        class: "profile-body",
                        h3 {
                            class: "username",
                            "Username Here"
                        },
                        p {
                            class: "status",
                            "This is a status message"
                        },
                        Button {
                            text: "Edit Profile".to_string(),
                            icon: Shape::PencilAlt,
                            on_pressed: move |_| {},
                        },
                        div {
                            class: "meta",
                            div {
                                class: "badges",
                                "badges"
                            },
                            div {
                                class: "location",
                                "Somewhere, USA"
                            },
                            div {
                                class: "friend-count",
                                "33"
                            }
                        }
                    }
                }
            ))
        },
    })
}
