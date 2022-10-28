use dioxus::prelude::*;
use dioxus_html::KeyCode;

// depends on javascript to resize the textarea
// because this is used in multiple places, the javascript was moved outside this Element
#[inline_props]
#[allow(non_snake_case)]
pub fn TextArea<'a>(
    cx: Scope,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    placeholder: String,
) -> Element<'a> {
    let clearing_state = &*cx.use_hook(|_| std::cell::Cell::new(false));
    cx.render(rsx! {
        textarea {
            class: "input resizeable-textarea",
            oninput: move |e| {
                if !clearing_state.get() {
                    text.set(e.value.clone());
                } else {
                    clearing_state.set(false);
                }
            },
            onkeydown: move |evt| {
                if evt.key_code == KeyCode::Enter && !evt.shift_key {
                    on_submit.call(text.to_string());
                    text.set(String::from(""));
                    clearing_state.set(true);
                }
            },
            placeholder: "{placeholder}",
            value: "{text}"
        }
    })
}
