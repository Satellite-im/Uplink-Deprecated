use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::icon_button::IconButton;

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
    let show_children = use_state(&cx, || true);

    let full_class = match full.get() {
        true => "popup full",
        false => "popup",
    };

    let hidden_class = match cx.props.hidden {
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
                    // TODO:
                    // ID:
                    // Title: Allow draging of popup handle to resize
                    // Reporter: Matt Wisniewski
                    // Desc:
                    // We should be able to click and drag the popup and snap the popup to different sizes.
                    onclick: move |evt| {
                        evt.cancel_bubble();
                        full.set(!full.get());
                    }
                }
                div {
                    class: "wrapper",
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
                                true => Shape::ArrowsPointingIn,
                                false => Shape::ArrowsPointingOut
                            }
                        },
                        IconButton {
                            on_pressed: move |_| {
                                cx.props.on_dismiss.call(());
                            },
                            icon: Shape::XMark
                        },
                    },
                    // TODO:
                    // ID:
                    // Title: Popup renders content forever
                    // Reporter: Matt Wisniewski
                    // Desc:
                    // We currently render the children even when the popup is hidden off screen.
                    // We are animating this popup so we need to make sure it's off screen before
                    // we "de-render" thse children, realistically this probably involves a 0.2ms delay
                    // followed by changing the children to render conditionally.
                    // Maybe something like...
                    // cx.spawn({
                    //     async move {
                    //         loop {
                    //             wait_ms(200).await;
                    //             show_children.set(false);
                    //         }
                    //     }
                    // })
                    show_children.then(|| rsx!(cx.props.children.as_ref()))
                }
            }
        }
    ))
}
