use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::button::Button;

#[inline_props]
#[allow(non_snake_case)]
pub fn Popout<'a>(
    cx: Scope,
    is_visible: UseState<bool>,
    remote: String,
    children: Element<'a>,
) -> Element<'a> {
    // Log a debug message
    log::debug!("rendering Popout");

    if !is_visible.get() {
        return None;
    }

    cx.render(rsx! {
       div {
            id: "popout",
            div {
                class: "popout-mask {remote}",
                children,
                div {
                    class: "close",
                    Button {
                        icon: Shape::XMark,
                        on_pressed: move |_| {
                            is_visible.set(false);
                        }
                    },
                },
            },
        }
    })
}
