use dioxus::prelude::*;
use sir::global_css;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    #[props(optional)]
    large: Option<bool>,
}

#[allow(non_snake_case)]
pub fn InlineSkeleton(cx: Scope<Props>) -> Element {
    global_css!(
        "
        @keyframes skeleton-loading {
            0% {
            background-color: var(--theme-text-muted);
            }
            100% {
            background-color: var(--theme-text-darker);
            }
        }

        .inline-skeleton {
            width: 100%;
            height: 20px;
            border-radius: 4px;
            animation: skeleton-loading 1s linear infinite alternate;
        }
    "
    );

    cx.render(rsx! {
        div {
            class: "inline-skeleton",
        }
    })
}
