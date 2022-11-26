use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::outline::Shape;
use fermi::use_atom_ref;
use ui_kit::icon_button::IconButton;

use crate::components::{
    main::files::toolbar::usage::{Usage, UsageStats},
    reusable::toolbar,
};
use crate::{state::Actions, STATE};

pub mod usage;

#[derive(Props)]
pub struct Props<'a> {
    on_new_folder: EventHandler<'a, MouseEvent>,
    on_show_upload: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Toolbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let st = use_atom_ref(&cx, STATE).clone();

    cx.render(rsx! {
        toolbar::Toolbar {
            controls: cx.render(rsx! {
                IconButton {
                    icon: Shape::ArchiveBoxArrowDown,
                    state: ui_kit::icon_button::State::Secondary,
                    on_pressed: move |_| {}
                },
                IconButton {
                    icon: Shape::FolderArrowDown,
                    state: ui_kit::icon_button::State::Secondary,
                    on_pressed: move |e| cx.props.on_new_folder.call(e)
                },
                IconButton {
                    icon: Shape::ArrowUpTray,
                    on_pressed: move |e| cx.props.on_show_upload.call(e)
                }
            }),
            div {
                class: "usage-container",
                div {
                    class: "mobile-back-button",
                    IconButton {
                        icon: Shape::ArrowLeft,
                        state: ui_kit::icon_button::State::Secondary,
                        on_pressed: move |_| {
                            st.write().dispatch(Actions::HideSidebar(false));
                        },
                    },
                },
                Usage {
                    usage: UsageStats {
                        available: 1256,
                        total: 123456,
                        used: 122200,
                        percent_free: 75,
                    }
                },
            },
        },
    })
}
