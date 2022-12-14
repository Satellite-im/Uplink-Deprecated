use dioxus::prelude::*;
use ui_kit::button::Button;

use crate::{components::reusable::popout::Popout, LANGUAGE};

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
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let handleConfirmation = move |_| {
        is_visible.set(false);
        on_confirm.call(());
    };

    cx.render(rsx! {
       Popout {
           is_visible: is_visible.clone(),
           remote: "confirmation-modal".to_string(),
           hide_close_button: true,
           div {
               id: "confirmation-modal",
               h3 {
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
                       text: l.cancel.to_string(),
                       state: ui_kit::button::State::Danger,
                       on_pressed: move |_| {
                          is_visible.set(false);
                       }
                   },
                   Button {
                       text: l.confirm.to_string(),
                       on_pressed: handleConfirmation,
                   },
               },
           },
       }
    })
}
