use chrono_humanize::HumanTime;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::raygun::Message;

use crate::components::ui_kit::{
    icon_button::{self, IconButton},
    icon_input::IconInput,
    input::Input,
};

#[derive(PartialEq, Props)]
pub struct Props {
    message: Message,
    remote: bool,
    first: bool,
    middle: bool,
    last: bool,
}

#[allow(non_snake_case)]
pub fn Msg(cx: Scope<Props>) -> Element {
    let popout = use_state(&cx, || false);
    let value = cx.props.message.clone().value().join("\n");
    let value2 = value.clone();
    let timestamp = cx.props.message.clone().date();
    let ht = HumanTime::from(timestamp);
    let remote = match cx.props.remote {
        true => "remote",
        false => "local",
    };
    let remote2 = remote.clone();
    let first = match cx.props.first {
        true => "first",
        false => "",
    };
    let first2 = first.clone();
    let middle = match cx.props.middle {
        true => "middle",
        false => "",
    };
    let middle2 = middle.clone();
    let last = match cx.props.last {
        true => "last",
        false => "",
    };
    let last2 = last.clone();

    cx.render(rsx! (
        div {
            class: "wrapper {remote}",
            (popout).then(|| rsx!(
                div {
                    class: "popout-mask {remote2}",
                    onclick: move |_| {
                        popout.set(false);
                    },
                    div {
                        class: "message-wrap",
                        div {
                            class: "user-message",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            div {
                                class: "pfp",
                            },
                            div {
                                class: "value popout {first2} {middle2} {last2}",
                                p {
                                    "{value2}"
                                },
                            },
                        }
                        div {
                            class: "controls",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            IconButton {
                                icon: Shape::EmojiHappy,
                                on_pressed: move |_| {}
                            },
                            IconInput {
                                icon: Shape::Reply,
                                value: "".to_string(),
                                placeholder: "Send a reply..".to_string(),
                                on_change: move |_| {},
                                on_enter: move |_| {}
                            }
                            IconButton {
                                icon: Shape::ArrowRight,
                                state: icon_button::State::Secondary,
                                on_pressed: move |_| {}
                            },
                        }
                    }
                }
            )),
            div {
                class: "message {remote}",
                if cx.props.remote {
                    rsx! (
                        if cx.props.last {
                            rsx!(
                                div {
                                    class: "pfp",
                                },
                            )
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                        div {
                            class: "value {first} {middle} {last}",
                            onclick: |_| {
                                popout.set(true);
                            },
                            p {
                                "{value}"
                            }
                        }
                    )
                } else {
                    rsx!(
                        div {
                            class: "value {first} {middle} {last}",
                            onclick: |_| {
                                popout.set(true);
                            },
                            p {
                                "{value}"
                            }
                        },
                        if cx.props.last {
                            rsx!(
                                div {
                                    class: "pfp",
                                },
                            )
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                    )
                }
                if cx.props.last {
                    rsx!(
                        div {
                            class: "timestamp",
                            "{ht}"
                        }
                    )
                } else {
                    rsx!( div {} )
                }
            }
        }
    ))
}
