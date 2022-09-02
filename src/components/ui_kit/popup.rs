use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

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
