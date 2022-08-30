use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>,
}

pub fn css() -> String {
    "
    .bar {
        float: left;
        width: 15px;
        height: 6px;
        border-radius: 2px;
        background-color: var(--theme-text-bright);
    }
    .load .bar {
        animation: loadingJ 2s cubic-bezier(0.17, 0.37, 0.43, 0.67) infinite;
    }
    .load span {
        color: var(--theme-placeholder);
        animation: loadingK 2s cubic-bezier(0.17, 0.37, 0.43, 0.67) infinite;
        font-family: 'Space Mono', monospace;
        font-size: 12pt;
    }
    @keyframes loadingK {
        0%,
        100% {
            color: var(--theme-placeholder);
        }
        50% {
            color: var(--theme-secondary);
        }
    }
    @keyframes loadingJ {
        0%,
        100% {
            transform: translate(0, 0);
        }
        50% {
            transform: translate(80px, 0);
            background-color: var(--theme-secondary);
            width: 25px;
        }
    }
    "
    .to_string()
}

#[allow(non_snake_case)]
pub fn Loader(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "load",
            span {
                cx.props.text.clone()
            },
            div {
                class: "bar"
            }
        }
    })
}
