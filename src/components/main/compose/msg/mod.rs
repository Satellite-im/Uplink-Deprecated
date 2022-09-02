use chrono_humanize::HumanTime;
use dioxus::prelude::*;
use warp::raygun::Message;

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
    let value = cx.props.message.clone().value().join("\n");
    let timestamp = cx.props.message.clone().date();
    let ht = HumanTime::from(timestamp);
    let remote = match cx.props.remote {
        true => "remote",
        false => "local",
    };
    let first = match cx.props.first {
        true => "first",
        false => "",
    };
    let middle = match cx.props.middle {
        true => "middle",
        false => "",
    };
    let last = match cx.props.last {
        true => "last",
        false => "",
    };

    cx.render(rsx! (
        div {
            class: "message {remote} {last}",
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
                        p {
                            "{value}"
                        }
                    }
                )
            } else {
                rsx!(
                    div {
                        class: "value {first} {middle} {last}",
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
    ))
}
