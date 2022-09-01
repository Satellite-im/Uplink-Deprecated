use dioxus::prelude::*;
use sir::global_css;

#[allow(non_snake_case)]
pub fn Badge(cx: Scope) -> Element {
    global_css!(
        "
        .badge-renderer {
            width: 24px;
            height: 24px;
            border-radius: 2px;
            background: var(--theme-text-muted);
            display: inline-block;
            margin: 0.25rem;
        }
    "
    );

    cx.render(rsx! {
        div {
            class: "badge-renderer",
            
        }
    })
}
