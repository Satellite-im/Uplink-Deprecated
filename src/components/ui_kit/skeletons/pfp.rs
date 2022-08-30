use dioxus::prelude::*;
use sir::global_css;

#[derive(PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    large: Option<bool>,
}

#[allow(non_snake_case)]
pub fn PFPSkeleton(cx: Scope<Props>) -> Element {
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

        .pfp-skeleton {
            height: 40px;
            width: 40px;
            border-radius: 20px;
            animation: skeleton-loading 1s linear infinite alternate;
        }
    "
    );

    cx.render(rsx! {
        div {
            class: "pfp-skeleton",
        }
    })
}
