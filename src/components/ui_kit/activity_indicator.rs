use dioxus::prelude::*;
use sir::global_css;


#[derive(PartialEq, Props)]
pub struct Props {
    inline: bool,
}

#[allow(non_snake_case)]
pub fn ActivityIndicator(cx: Scope<Props>) -> Element {
    global_css!("
        .activity.inline {
            display: inline-flex;
            align-items: center;

            p {
                font-size: 10pt;
                color: var(--theme-text-darker);
                font-family: 'Space Mono', monospace !important;
            }
            .bubble {
                height: 12px;
                width: 12px;
                background: var(--theme-text-muted);
                border-radius: 6px;
                margin-right: 0.5rem;
            }
        }
    ");

    let main_class = match cx.props.inline {
        true => "inline",
        false => "icon-icon",
    };

    cx.render(rsx! {
        div {
            class: "activity {main_class}",
            div { class: "bubble" },
            p {
                "Unknown"
            }
        }
    })
}
