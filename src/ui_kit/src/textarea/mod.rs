use crate::{
    context_menu::{ContextItem, ContextMenu},
    utils,
};
use dioxus::prelude::*;
use dioxus_elements::input_data::keyboard_types::{Code, Modifiers};

// for more information about this, see here: https://github.com/DioxusLabs/dioxus/issues/611
// `text` is passed in this way because it is lifted. This allows for a 'send' button to clear the text
#[inline_props]
#[allow(non_snake_case)]
//TODO: Evaluate inner_html and `cx.use_hook(|_| " ").clone();` to determine if this is actually necessary
#[allow(clippy::clone_double_ref)]
//TODO: Like above but for `inner_html = " "`
#[allow(unused)]
pub fn TextArea<'a>(
    cx: Scope,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    placeholder: String,
) -> Element<'a> {
    log::debug!("rendering ui_kit/TextArea");

    let clearing_state = &*cx.use_hook(|| std::cell::Cell::new(false));

    let mut inner_html = cx.use_hook(|| " ").clone();
    if clearing_state.get() {
        inner_html = "";
        cx.needs_update();
    }

    let formatted = utils::wrap_in_markdown(text.as_ref());

    let elm = rsx! {
        div {
            class: "textarea-wrap",
            id: "TODO-textarea-input",
            ContextMenu {
                parent: String::from("TODO-textarea-input"),
                items: cx.render(rsx! {
                    ContextItem {
                        onpressed: move |_| {},
                        text: String::from("Paste"),
                    },
                    ContextItem {
                        onpressed: move |_| {},
                        text: String::from("Select All"),
                    },
                    ContextItem {
                        onpressed: move |_| {},
                        text: String::from("Copy"),
                    },
                    ContextItem {
                        onpressed: move |_| {},
                        text: String::from("Clear"),
                    },
                })
            },
            (text.is_empty()).then(|| rsx!{
                span {
                    class: "placeholder",
                    "{placeholder}"
                },
            }),
            div {
                class: "shadow-input",
                "dangerous_inner_html": "{formatted}"
            },
            div {
                class: "dynamic-input",
                contenteditable: "true",
                oninput: move |e| {
                    if !clearing_state.get() {
                        text.set(e.value.clone());
                    } else {
                        clearing_state.set(false);
                    }
                },
                onkeyup: |e| {
                    if e.data.code().eq(&Code::Enter) && !e.data.modifiers().eq(&Modifiers::SHIFT) {
                        if !text.trim().is_empty() {
                            on_submit.call(text.trim().to_string());
                        }
                        text.set(String::from(""));
                        clearing_state.set(true);
                    }
                },
                "dangerous_inner_html": "{inner_html}"
            }
        }
    };

    clearing_state.set(false);
    cx.render(elm)
}
