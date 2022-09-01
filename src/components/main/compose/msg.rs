use dioxus::prelude::*;
use sir::global_css;
use warp::raygun::Message;
use chrono_humanize::HumanTime;

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
    global_css!("
        .message {
            width: max-content;
            display: inline-flex;
            align-self: flex-end;
            margin: 0.25rem 0;
            position: relative;

            &.last {
                margin-bottom: 1rem;

            }

            .pfp {
                height: 40px;
                width: 40px;
                border-radius: 20px;
                background: var(--theme-text-muted);
            }
            .pfp-void {
                height: 40px;
                width: 40px;
            }
            .value {
                flex: 1;
                padding: 0.5rem 1rem;
                border-radius: 16px 16px 4px 16px;
                background-color: var(--theme-primary);
                margin-right: 1rem;
                margin-left: 0;
                &.first {
                    border-radius: 16px 16px 4px 16px;
                }
                &.middle {
                    border-radius: 16px 4px 4px 16px;
                }
                &.last {
                    border-radius: 16px 4px 16px 16px;
                    margin-bottom: 1rem;
                }
                p {
                    font-size: 12px;
                    margin: 0;
                }
            }

            .timestamp {
                position: absolute;
                bottom: -0.5rem;
                right: calc(40px + 1rem);
                font-size: 10px;
                color: var(--theme-text-muted);
            }
            
            &.remote {
                align-self: flex-start;
                .value {
                    background: var(--theme-foreground-dark);
                    border-radius: 16px 16px 16px 4px;
                    margin-left: 1rem;
                    margin-right: 0;
                    &.first {
                        border-radius: 16px 16px 16px 4px;
                    }
                    &.middle {
                        border-radius: 4px 16px 16px 4px;
                    }
                    &.last {
                        border-radius: 4px 16px 16px 16px;
                    }
                }
                .timestamp {
                    left: calc(40px + 1rem);
                    right: unset;
                    text-align: right;
                }
            }
        }
    ");

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
