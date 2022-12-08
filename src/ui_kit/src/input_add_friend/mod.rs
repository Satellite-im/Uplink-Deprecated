use crate::context_menu::{ContextItem, ContextMenu};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_elements::KeyCode;

#[inline_props]
#[allow(non_snake_case)]
pub fn InputAddFriend<'a>(
    cx: Scope,
    placeholder: String,
    value: UseState<String>,
    on_change: EventHandler<'a, FormEvent>,
    on_enter: EventHandler<'a, String>,
) -> Element<'a> {
    let clearing_state = &*cx.use_hook(|_| std::cell::Cell::new(false));
    let inner_html = cx.use_hook(|_| " ");
    if !inner_html.is_empty() && value.is_empty() && !clearing_state.get() {
        *inner_html = "";
        clearing_state.set(true);
        cx.needs_update();
    } else {
        clearing_state.set(false);
    }

    let res = rsx! {
        div{
            class: "input-add-friend",
            id:"input-add-friend",
            ContextMenu {
                parent: String::from("input-add-friend"),
                items: cx.render(rsx! {
                    // TODO: copy paste feature
                    // ContextItem {
                    //     onpressed: move |_| {
                    //         let mut clipboard = Clipboard::new().unwrap();
                    //         let copied_text = clipboard.get_text();
                    //         match copied_text {
                    //             Ok(v)=> {println!("text:{}",v);
                    //         }
                    //             Err(e)=> println!("Paste text err:{}",e),
                    //         }
                    //     },
                    //     text: String::from("Paste"),
                    // },
                    // ContextItem {
                    //     onpressed: move |_| {},
                    //     text: String::from("Select All"),
                    // },
                    // ContextItem {
                    //     onpressed: move |_| {},
                    //     text: String::from("Copy"),
                    // },
                    ContextItem {
                        onpressed: move |_| value.set(String::from("")),
                        text: String::from("Clear"),
                    },
                })
            },
            (value.is_empty()).then(|| rsx!{
                span {
                    class: "add-friend-placeholder",
                    "{placeholder}"
                },
            }),
            div {
                class: "textfield-add-friend",
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
            },
        }
    };

    cx.render(res)
}
