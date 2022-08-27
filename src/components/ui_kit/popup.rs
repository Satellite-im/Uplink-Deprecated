use dioxus::prelude::*;
use sir::global_css;
use warp::tesseract::Tesseract;

#[derive(Props)]
pub struct Props<'a> {
    tesseract: Tesseract,
    children: Element<'a>,
    close: EventHandler<'a>,
}

#[allow(non_snake_case)]
pub fn Popup<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let full = use_state(&cx, || false);

    global_css! {"
        .popup-mask {
            -webkit-backdrop-filter: blur(3px);
            position: absolute;
            top: 0;
            right: 0;
            bottom: 0;
            left: 0;
            z-index: 90;
            
            .popup {
                align-self: center;
                min-height: 150px;
                border-radius: 8px 8px 0 0;
                position: absolute;
                left: 5px;
                right: 5px;
                bottom: 0;
                max-height: 200px;
                padding: 1rem;
                display: flex;
                flex-direction: column;
                transition: height 2s ease 0s;
                background: var(--theme-modal);


                .handle {
                    content: '';
                    cursor: pointer;
                    pointer-events: all;
                    position: absolute;
                    width: 70%;
                    left: 15%;
                    height: 6px;
                    background: var(--theme-placeholder);
                    border-radius: 3px;
                    top: -15px;
                    border: none;
                    z-index: 100;
                }

                .title {
                    text-align: left;
                    align-content: center;
                    display: inline-flex;
                    border-bottom: 1px solid var(--theme--borders);
                    color: var(--theme-text);

                    svg {
                        margin-right: 1rem;
                        stroke: var(--theme-text);
                    }
                }
            }
            .popup_full {
                transition: max-height 0.2s ease;
                max-height: 90%;
                min-height: 50%;
            }
        }
    "}

    let full_class = match full.get() {
        true => "popup popup_full",
        false => "popup",
    };

    cx.render(rsx!(
        div {
            class: "popup-mask",
            onclick: move |_| {
                cx.props.close.call(());
            },
            div {
                class: "{full_class}",
                button {
                    class: "handle",
                    onclick: move |_| {
                        full.set(!full.get());
                    }
                }
                cx.props.children.as_ref()
            }
        }
    ))
}