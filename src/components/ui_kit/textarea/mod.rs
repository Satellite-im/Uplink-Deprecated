use crate::utils;
use dioxus::prelude::*;
use dioxus_html::KeyCode;

// `text` is passed in this way because it is lifted. This allows for a 'send' button to clear the text
#[inline_props]
#[allow(non_snake_case)]
//TODO: Evaluate inner_html and `cx.use_hook(|_| " ").clone();` to determine if this is actually necessary 
#[allow(clippy::clone_double_ref)]
//TODO: Like above but for `inner_html = " "`
#[allow(unused_assignments)]
pub fn TextArea<'a>(
    cx: Scope,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    placeholder: String,
) -> Element<'a> {
    let clearing_state = &*cx.use_hook(|_| std::cell::Cell::new(false));

    let mut inner_html = cx.use_hook(|_| " ").clone();
    if clearing_state.get() {
        inner_html = "";
        cx.needs_update();
    }

    let formatted = utils::wrap_in_markdown(text.as_ref());

    let elm = rsx! {
        div {
            class: "textarea-wrap",
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
                    if e.data.key_code.eq(&KeyCode::Enter) && !e.data.shift_key {
                        on_submit.call(text.to_string());
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
