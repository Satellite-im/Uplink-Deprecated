use crate::{
    context_menu::{ContextItem, ContextMenu},
    utils,
};
use dioxus::prelude::*;
use dioxus_html::KeyCode;

// for more information about this, see here: https://github.com/DioxusLabs/dioxus/issues/611
// `text` is passed in this way because it is lifted. This allows for a 'send' button to clear the text
#[inline_props]
#[allow(non_snake_case)]
//TODO: Evaluate inner_html and `cx.use_hook(|_| " ").clone();` to determine if this is actually necessary
#[allow(clippy::clone_double_ref)]
//TODO: Like above but for `inner_html = " "`
#[allow(unused_assignments)]
pub fn TextArea<'a>(
    cx: Scope,
    on_input: EventHandler<'a, String>,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    placeholder: String,
) -> Element<'a> {
    log::debug!("rendering ui_kit/TextArea");

    let clear_disabled = &*cx.use_hook(|_| std::cell::Cell::new(true));
    let formatted = utils::wrap_in_markdown(text.as_ref());
    let mut inner_html = cx.use_hook(|_| " ").clone();

    if !inner_html.is_empty() && text.is_empty() && !clear_disabled.get() {
        inner_html = "";
        clear_disabled.set(true);
        cx.needs_update();
    } else {
        clear_disabled.set(false);
    }

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
                        text.set(e.value.clone());
                        on_input.call(e.value.clone());
                },
                onkeyup: |e| {
                    if e.data.key_code.eq(&KeyCode::Enter) && !e.data.shift_key {
                        if !text.trim().is_empty() {
                            on_submit.call(text.trim().to_string());
                        }
                        text.set(String::from(""));
                    }
                },
                "dangerous_inner_html": "{inner_html}"
            }
        }
    };

    cx.render(elm)
}
