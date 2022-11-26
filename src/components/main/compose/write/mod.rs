use crate::{iutils::config::Config, LANGUAGE};
// use audio_factory::AudioFactory;
// use utils::extensions::BasicExtension;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use fermi::use_atom_ref;
use ui_kit::{
    context_menu::{ContextItem, ContextMenu},
    icon_button::{self, IconButton},
    small_extension_placeholder::SmallExtensionPlaceholder,
    textarea::TextArea,
};

#[derive(Props)]
pub struct Props<'a> {
    on_submit: EventHandler<'a, String>,
    on_upload: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Write<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering compose/Write");
    let config = Config::load_config_or_default();
    let text = use_state(&cx, String::new);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    cx.render(rsx! {
        div {
            class: "write",
            id: "write",
            ContextMenu {
                parent: String::from("write"),
                items: cx.render(rsx! {
                    ContextItem {
                        onpressed: move |_| {},
                        icon: Shape::Clipboard,
                        text: String::from("Copy Conversation ID")
                    },
                })
            },
            IconButton {
                icon: Shape::Plus,
                on_pressed: move |_| {
                    let _ = &cx.props.on_upload.call(());
                },
            },
            TextArea {
                on_submit: |val| cx.props.on_submit.call(val),
                text: text.clone(),
                placeholder: l.chatbar_placeholder.to_string()
            }
            config.developer.developer_mode.then(|| rsx! {
                div {
                    class: "extension-holder",
                    SmallExtensionPlaceholder {}
                }
            })
            // div {
            //     class: "chatbar_extensions",
            //     // AudioFactory::render()
            // },
            div {
                id: "send",
                IconButton {
                    icon: Shape::ArrowRight,
                    state: icon_button::State::Secondary,
                    on_pressed: move |_| {
                        let text = text.clone();
                        let _ = &cx.props.on_submit.call(text.to_string());
                        text.set(String::from(""));
                    },
                }
            }
        }
    })
}
