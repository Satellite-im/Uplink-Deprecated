use dioxus::prelude::*;
use dioxus_html::KeyCode;

// `text` is passed in this way because it is lifted. This allows for a 'send' button to clear the text
#[inline_props]
#[allow(non_snake_case)]
pub fn TextArea<'a>(
    cx: Scope,
    on_submit: EventHandler<'a, String>,
    text: UseState<String>,
    placeholder: String,
) -> Element<'a> {
    let clearing_state = &*cx.use_hook(|_| std::cell::Cell::new(false));

    let inner_html = cx.use_hook(|_| " ");
    if clearing_state.get() {
        *inner_html = "";
        cx.needs_update();
    }

    let elm = rsx! {
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
            "placeholder": "{placeholder}",
            "dangerous_inner_html": "{inner_html}"
        }
    };

    clearing_state.set(false);
    *inner_html = " ";
    cx.render(elm)
}
