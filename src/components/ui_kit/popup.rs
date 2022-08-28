use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use crate::components::ui_kit::icon_button::IconButton;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    on_dismiss: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Popup<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let full = use_state(&cx, || false);

    global_css! {"
        .popup-mask {
            -webkit-backdrop-filter: blur(3px);
            background: var(--theme-semi-transparent);
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
                transition-property: min-height;
                transition-duration: 0.2s;
                background: var(--theme-modal);

                .controls {
                    position: absolute;
                    top: 0;
                    right: 0;
                    
                    .button {
                        background: transparent;
                        color: var(--theme-muted);
                    }
                }

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
            onclick: move |_| cx.props.on_dismiss.call(()),
            div {
                class: "{full_class}",
                button {
                    class: "handle",
                    // TODO: This handle should be able to be "grabbed" and "pulled" up or down to expand or close the opup
                    onclick: move |evt| {
                        evt.cancel_bubble();
                        full.set(!full.get());
                    }
                }
                div {
                    onclick: move |evt| {
                        evt.cancel_bubble();
                        full.set(true);
                    },
                    div {
                        class: "controls",
                        IconButton {
                            on_pressed: move |_| {},
                            // TODO: This button should "pop" the "popup" out into a floating centered modal.
                            // TODO: Less important, it should have some kind of animation tied to this
                            // Disabled pending impl
                            disabled: true,
                            icon: Shape::ArrowsExpand,
                        },
                    },
                    cx.props.children.as_ref()
                }
            }
        }
    ))
}