use chrono_humanize::HumanTime;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::raygun::Message;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        icon_textarea::IconTextArea,
    },
    LANGUAGE,
};

#[derive(Props)]
pub struct Props<'a> {
    message: Message,
    remote: bool,
    first: bool,
    middle: bool,
    last: bool,
    on_reply: EventHandler<'a, String>,
}

#[allow(non_snake_case)]
pub fn Msg<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let popout = use_state(&cx, || false);
    // text has been lifted from the child components into Msg so that
    // a button press can be used to clear it.
    let text = use_state(&cx, String::new);
    let text2 = text.clone();
    let value = cx.props.message.clone().value().join("\n");
    let value2 = value.clone();
    let timestamp = cx.props.message.clone().date();
    let ht = HumanTime::from(timestamp);
    let remote = match cx.props.remote {
        true => "remote",
        false => "local",
    };
    let l = use_atom_ref(&cx, LANGUAGE).read();

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

    let hover = use_state(&cx, || false);

    let hover_class = match hover.get() {
        true => "animate_animated animate__pulse",
        false => "not-hovered",
    };

    let slide_class = match cx.props.remote {
        true => "message-wrap animate__animated animate__pulse animate__slideInLeft",
        false => "message-wrap animate__animated animate__pulse animate__slideInRight",
    };

    // TODO: This is ugly, but we need it for resizing textareas until someone finds a better solution.
    // note that this has a queryselector and click handler specific to this page
    const RESIZE_TEXTAREA_SCRIPT: &str = r#"
    (function addAutoResize() {
        let element = document.querySelector('.reply-container .resizeable-textarea');
        if (element == null) {
            return;
        }

        element.style.boxSizing = 'border-box';
        var offset = element.offsetHeight - element.clientHeight;
        element.addEventListener('input', function (event) {
            event.target.style.height = 'auto';
            event.target.style.height = event.target.scrollHeight + offset + 'px';
        });
        element.removeAttribute('data-autoresize');
    })()"#;

    cx.render(rsx! (
        div {
            class: "wrapper {remote}",
            (popout).then(|| rsx!(
                div {
                    class: "popout-mask {remote}",
                    onclick: move |_| {
                        popout.set(false);
                    },
                    div {
                        class: "close",
                        IconButton {
                            icon: Shape::X,
                            on_pressed: move |_| {
                                popout.set(false);
                            }
                        },
                    },
                    div {
                        class: "message-wrap {slide_class}",
                        div {
                            class: "user-message",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            div {
                                class: "pfp",
                            },
                            div {
                                class: "value popout {first} {middle} {last}",
                                p {
                                    "{value2}"
                                },
                            },
                        }
                        div {
                            class: "controls reply-container",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            IconButton {
                                icon: Shape::EmojiHappy,
                                on_pressed: move |_| {}
                            },
                            IconTextArea {
                                icon: Shape::Reply,
                                placeholder: l.send_a_reply.to_string(),
                                on_submit: move |e| {
                                    cx.props.on_reply.call(e);
                                    popout.set(false);
                                },
                                text: text.clone(),
                            },
                            IconButton {
                                icon: Shape::ArrowRight,
                                state: icon_button::State::Secondary,
                                on_pressed: move |_| {
                                    cx.props.on_reply.call(text.clone().to_string());
                                    popout.set(false);
                                }
                            },
                            script {
                                dangerous_inner_html: "{RESIZE_TEXTAREA_SCRIPT}"
                            },
                        }
                    }
                }
            )),
            div {
                class: "message {remote} {hover_class}",
                if cx.props.remote {
                    rsx! (
                        if cx.props.last {
                            rsx!( div { class: "pfp" } )
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                        div {
                            class: "value {first} {middle} {last}",
                            onclick: |_| {
                                popout.set(true);
                            },
                            onmouseover: |_| {
                                hover.set(true);
                            },
                            onmouseout: |_| {
                                hover.set(false);
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
                            onmouseover: |_| {
                                hover.set(true);
                            },
                            onmouseout: |_| {
                                hover.set(false);
                            },
                            p {
                                "{value}"
                            }
                        },
                        if cx.props.last {
                            rsx!( div { class: "pfp" } )
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                    )
                }
                cx.props.last.then(|| rsx!(
                    div {
                        class: "timestamp",
                        "{ht}"
                    }
                ))
            }
        }
    ))
}
