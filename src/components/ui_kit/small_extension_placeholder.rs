use dioxus::prelude::*;
use sir::global_css;

#[allow(non_snake_case)]
pub fn SmallExtensionPlaceholder(cx: Scope) -> Element {
    global_css!(
        "
        .mini-extension-renderer {
            width: calc(40px - 1rem);
            height: calc(40px - 1rem);
            display: inline-flex;
            flex-direction: column;
            align-content: center;
            justify-content: center;
            border-radius: 4px;
            font-size: 10px;
            color: var(--theme-text-muted);
            border: 1px dashed var(--theme-borders);
            padding: 0.5rem;

            button {
                border: none;
                color: var(--theme-text-muted);
                background: var(--theme-borders);
                border-radius: 4px;
                margin: 0 auto;
            }

        }
    "
    );

    cx.render(rsx! {
        div {
            class: "mini-extension-renderer",
            button {
                "+"
            }
        }
    })
}
