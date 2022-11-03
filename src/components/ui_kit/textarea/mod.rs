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
    let dyn_input = rsx! {
        div {
            class: "dynamic-input",
            "key": "{text}",
            oninput: |e| {
                if !clearing_state.get() {
                    text.set(e.value.clone());
                } else {
                    clearing_state.set(false);
                }
            },
            onkeyup: |e| {
                if e.data.key_code.eq(&KeyCode::Enter) && !e.data.shift_key {
                    cx.props.on_submit.call(text.to_string());
                    text.set(String::from(""));
                    clearing_state.set(true);
                }
            },
            "contenteditable": "true",
            "placeholder": "{placeholder}"
        }
    };

    cx.render(dyn_input)
}
