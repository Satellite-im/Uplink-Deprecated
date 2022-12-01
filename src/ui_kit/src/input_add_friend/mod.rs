use dioxus::{events::FormEvent, prelude::*};
use dioxus_elements::KeyCode;

#[inline_props]
#[allow(non_snake_case)]
pub fn InputAddFriend<'a>(
    cx: Scope,
    value: UseState<String>,
    on_change: EventHandler<'a, FormEvent>,
    on_enter: EventHandler<'a, String>,
) -> Element<'a> {
    let clearing_state = &*cx.use_hook(|_| std::cell::Cell::new(false));

    let mut inner_html = cx.use_hook(|_| " ").clone();
    if !inner_html.is_empty() && value.is_empty() && !clearing_state.get() {
        inner_html = "";
        clearing_state.set(true);
        cx.needs_update();
    } else {
        clearing_state.set(false);
    }

    let res = rsx! {
            div {
                class: "input-add-friend",
                contenteditable: "true",
                oninput: move |e| {
                    value.set(e.value.clone());
                    on_change.call(e);
                },
                onkeyup: |e| {
                    if e.data.key_code.eq(&KeyCode::Enter){
                        on_enter.call(value.to_string());
                        value.set(String::from(""));
                    }
                },
                "dangerous_inner_html": "{inner_html}",
            }
    };

    cx.render(res)
}
