use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::button::Button;

use crate::components::reusable::popout::Popout;

#[inline_props]
#[allow(non_snake_case)]
pub fn ConfirmationModal<'a>(
    cx: Scope,
    title: String,
    description: String,
    is_visible: UseState<bool>,
    on_confirm: EventHandler<'a, ()>,
) -> Element<'a> {
    // Log a debug message
    log::debug!("rendering ConfirmationModal");

    let handleConfirmation = move |_| {
        is_visible.set(false);
        on_confirm.call(());
    };

    cx.render(rsx! {
       Popout {
           is_visible: is_visible.clone(),
           remote: "confirmation-modal".to_string(),
           div {
               id: "confirmation-modal",
               h2 {
                   class: "modal-title",
                   "{title}",
               },
               p {
                   class: "modal-description",
                   "{description}",
               },
               div {
                   class: "modal-actions",
                   Button {
                       icon: Shape::XMark,
                       state: ui_kit::button::State::Danger,
                       on_pressed: move |_| {
                           is_visible.set(false);
                       }
                   },
                   Button {
                       icon: Shape::Check,
                       on_pressed: handleConfirmation,
                   },
               },
           },
       }
    })
}
