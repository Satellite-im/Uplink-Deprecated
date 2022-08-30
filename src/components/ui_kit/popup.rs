use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use crate::components::ui_kit::icon_button::IconButton;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    on_dismiss: EventHandler<'a, ()>,
    hidden: bool,
}

#[allow(non_snake_case)]
pub fn Popup<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let full = use_state(&cx, || false);
    let modal = use_state(&cx, || false);

    global_css! {"
        .popup-mask {
            -webkit-backdrop-filter: blur(3px);
            transition: blur 0.2s;
            background: var(--theme-semi-transparent);
            position: absolute;
            top: 0;
            right: 0;
            bottom: 0;
            left: 0;
            z-index: 100;

            &.as-modal {
                position: fixed;
                
            }

            &.hidden {
                -webkit-backdrop-filter: none;
                background: transparent;
                pointer-events: none;
            }

            .popup {
                align-self: center;
                border-radius: 8px 8px 0 0;
                position: absolute;
                overflow: show;
                left: 5px;
                right: 5px;
                bottom: 0;
                display: flex;
                flex-direction: column;
                height: 50%;
                transition: height 0.2s;
                background: var(--theme-foreground);
                margin-top: 50px;

                &.full {
                    height: max-content;
                }
                &.hidden {
                    height: 0px;
                    padding: 0;
                    overflow: hidden;
                }
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
        }
    "};

    let full_class = match full.get() {
        true => "popup full",
        false => "popup",
    };

    let hidden_class = match cx.props.hidden.clone() {
        true => "hidden",
        false => "show",
    };

    let as_modal = match *modal.clone() {
        true => "as-modal",
        false => "",
    };

    cx.render(rsx!(
        div {
            class: "popup-mask {hidden_class} {as_modal}",
            onclick: move |_| cx.props.on_dismiss.call(()),
            div {
                class: "{full_class} {hidden_class}",
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
                            on_pressed: move |_| {
                                modal.set(!modal.clone());
                            },
                            icon: match *modal.clone() {
                                true => Shape::Minus,
                                false => Shape::ArrowsExpand
                            }
                        },
                    },
                    cx.props.children.as_ref()
                }
            }
        }
    ))
}
