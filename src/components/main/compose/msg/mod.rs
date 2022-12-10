use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use embeds::LinkEmbed;
use linkify::LinkFinder;
use pulldown_cmark::{html, Options, Parser};

use state::{Actions, STATE};
use ui_kit::{
    button::Button,
    context_menu::{ContextItem, ContextMenu},
    profile_picture::PFP,
};
use utils::Account;
use warp::{crypto::DID, raygun::Message};

use crate::{
    components::reusable::textarea::TextArea,
    iutils::{
        self,
        get_meta::{get_meta, SiteMeta},
    },
    Messaging, LANGUAGE,
};

mod attachment;
pub mod embeds;
use attachment::Attachment;

#[derive(Props)]
pub struct Props<'a> {
    message: Message,
    messaging: Messaging,
    account: Account,
    sender: DID,
    remote: bool,
    first: bool,
    middle: bool,
    last: bool,
    profile_picture: Option<String>,
    on_reply: EventHandler<'a, String>,
}

#[allow(non_snake_case)]
pub fn Msg<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    log::debug!("rendering compose/Msg");
    let state = use_atom_ref(&cx, STATE).clone();
    let finder = LinkFinder::new();
    let content = cx.props.message.value();
    let attachments = cx.props.message.attachments();
    let joined_a = content.join("\n");
    let joined_b = joined_a.clone();
    let has_links = finder.links(&joined_b).next().is_some();

    // Parses links and grabs data like the title, favicon and description
    let fetch_meta = use_future(&cx, &joined_a, |content| async move {
        if has_links {
            let s = content.as_str();

            let links: Vec<_> = finder.links(s).collect();

            let first_link = match links.first() {
                Some(l) => l.as_str(),
                None => "",
            };
            get_meta(first_link).await
        } else {
            Ok(SiteMeta::default())
        }
    });

    let meta = match fetch_meta.value() {
        Some(Ok(val)) => val.clone(),
        Some(Err(_)) => SiteMeta::default(),
        None => SiteMeta::default(),
    };
    let meta2 = meta.clone();

    let popout = use_state(&cx, || false);
    // text has been lifted from the child components into Msg so that
    // a button press can be used to clear it.
    let text = use_state(&cx, String::new);
    let value = cx.props.message.clone().value().join("\n");

    let timestamp = cx.props.message.clone().date();
    let ht = iutils::display_msg_time(timestamp);
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

    let profile_picture2 = cx.props.profile_picture.clone();
    let profile_picture3 = cx.props.profile_picture.clone();

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&value, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(value.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    let (output1, output2, output3) = (
        html_output.clone(),
        html_output.clone(),
        html_output.clone(),
    );

    let id = cx.props.message.id();

    let attachment_list = attachments.iter().map(|file| {
        let key = file.id();
        rsx!(Attachment {
            key: "{key}",
            file: file.clone(),
            message: cx.props.message.clone(),
        })
    });
    let attachment_list2 = attachment_list.clone();

    cx.render(rsx! (
        div {
            class: "wrapper {remote}",
            (popout).then(|| rsx!(
                div {
                    class: "popout-mask {remote}",
                    div {
                        class: "close",
                        Button {
                            icon: Shape::XMark,
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
                            PFP {
                                src: cx.props.profile_picture.clone(),
                                size: ui_kit::profile_picture::Size::Normal
                            },
                            div {
                                class: "value popout {first} {middle} {last}",
                                div {
                                    class: "message-content",
                                    dangerous_inner_html: "{output1}",
                                    has_links.then(|| rsx!{
                                        LinkEmbed {
                                            meta: meta2
                                        }
                                    }),
                                    div {
                                        attachment_list2
                                    }
                                },
                            },
                        }
                        div {
                            class: "controls reply-container",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            Button {
                                icon: Shape::FaceSmile,
                                on_pressed: move |_| {}
                            },
                            TextArea {
                                messaging: cx.props.messaging.clone(),
                                placeholder: l.send_a_reply.to_string(),
                                on_input: move |_| {}
                                on_submit: move |e| {
                                    cx.props.on_reply.call(e);

                                    popout.set(false);
                                },
                                text: text.clone(),
                            },
                            Button {
                                icon: Shape::ArrowRight,
                                state: ui_kit::button::State::Secondary,
                                on_pressed: move |_| {
                                    cx.props.on_reply.call(text.clone().to_string());
                                    popout.set(false);
                                }
                            }
                        }
                    }
                }
            )),
            div {
                class: "message {remote} {hover_class}",
                id: "{id}-message",
                ContextMenu {
                    parent: format!("{}-message", &id),
                    items: cx.render(rsx! {
                        if cx.props.remote {rsx !{
                            ContextItem {
                                onpressed: move |_| popout.set(true),
                                text: String::from("React"),
                                icon: Shape::FaceSmile,
                            },
                            ContextItem {
                                onpressed: move |_| popout.set(true),
                                text: String::from("Reply"),
                                icon: Shape::ArrowUturnLeft,
                            },
                            ContextItem {
                                onpressed: move |_| {
                                    state
                                    .write()
                                    .dispatch(Actions::HideConversation(cx.props.message.conversation_id()));
                                },
                                text: String::from("Hide Chat"),
                                danger: true,
                                icon: Shape::Trash,
                            },
                            ContextItem {
                                onpressed: move |_| {
                                    // when the FriendRemoved event is detected, the covnersation will be removed
                                    // todo: do we want to be able to delete and re-add a friend and keep the previous conversation? maybe the users won't care if they don't know they can have that feature. 
                                    let mut multipass = cx.props.account.clone();
                                    let did_to_remove = cx.props.sender.clone();
                                    if multipass.remove_friend(&did_to_remove).is_err() {
                                        log::debug!("error removing friend");
                                    }
                                },
                                text: String::from("Remove Friend"),
                                danger: true,
                                icon: Shape::XCircle,
                            }
                        }} else {rsx!{
                            ContextItem {
                                onpressed: move |_| popout.set(true),
                                text: String::from("React"),
                                icon: Shape::FaceSmile,
                            },
                            ContextItem {
                                onpressed: move |_| popout.set(true),
                                text: String::from("Reply"),
                                icon: Shape::ArrowUturnLeft,
                            },
                            ContextItem {
                                onpressed: move |_| popout.set(true),
                                text: String::from("Edit"),
                                icon: Shape::PencilSquare,
                            },
                            ContextItem {
                                onpressed: move |_| {},
                                danger: true,
                                icon: Shape::Trash,
                                text: String::from("Remove"),
                            },
                        }}
                    })
                },
                if cx.props.remote {
                    rsx! (
                        if cx.props.last {
                            rsx!(
                                span {
                                    id: "{id}-pfp-message",
                                    ContextMenu {
                                        parent: format!("{}-pfp-message", id),
                                        items: cx.render(rsx! {
                                            ContextItem {
                                                onpressed: move |_| {},
                                                text: String::from("View Profile"),
                                            },
                                        })
                                    },
                                    PFP {
                                        src: profile_picture2,
                                        size: ui_kit::profile_picture::Size::Normal
                                    }
                                }
                            )
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                        div { // todo: don't duplicate this
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
                            div {
                                dangerous_inner_html: "{output2}",
                                has_links.then(|| rsx!{
                                    LinkEmbed {
                                        meta: meta
                                    }
                                }),
                                div {
                                    attachment_list
                                }
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
                            div {
                                dangerous_inner_html: "{output3}",
                                has_links.then(|| rsx!{
                                    LinkEmbed {
                                        meta: meta
                                    }
                                }),
                                div {
                                    attachment_list
                                }
                            }
                        },
                        if cx.props.last {
                            rsx!(PFP {
                                src: profile_picture3,
                                size: ui_kit::profile_picture::Size::Normal
                            })
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
