use dioxus::prelude::*;
use sir::global_css;


#[allow(non_snake_case)]
pub fn ExtensionPlaceholder(cx: Scope) -> Element {
    global_css!("
        .extension-renderer {
            width: calc(100% - 1rem);
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
                max-width: 200px;
                color: var(--theme-text-muted);
                background: var(--theme-borders);
                border-radius: 4px;
                margin: 0 auto;
            }

        }
    ");

    cx.render(rsx!{
        div {
            class: "extension-renderer",
            "Sidebar Ext. Frame",
            button {
                "Get Extensions"
            }
        }
    })
}