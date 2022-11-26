use dioxus::prelude::*;
use fermi::use_atom_ref;

use crate::{
    components::main::settings::pages::{
        developer::Developer, extensions::Extensions, general::General, profile::Profile,
    },
    components::reusable::toolbar,
    state::Actions,
    Account, STATE,
};

use dioxus_heroicons::outline::Shape;
use ui_kit::icon_button::IconButton;

use self::sidebar::nav::Route;

pub mod pages;
pub mod sidebar;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    page_to_open: Route,
}

#[allow(non_snake_case)]
pub fn Settings(cx: Scope<Props>) -> Element {
    let st = use_atom_ref(&cx, STATE).clone();
    log::debug!("rendering Settings");
    let page_to_open_on_settings = match cx.props.page_to_open {
        Route::Profile => Route::Profile,
        Route::Developer => Route::Developer,
        _ => Route::General,
    };

    let sidebar_visibility = match st.read().hide_sidebar {
        false => "sidebar-visible",
        true => "sidebar-hidden",
    };

    let active_page = use_state(&cx, || page_to_open_on_settings);

    let active_page_string = match **active_page {
        Route::Profile => "Profile",
        Route::Privacy => "Privacy",
        Route::AudioVideo => "Audio Video",
        Route::Extensions => "Extensions",
        Route::Developer => "Developer",
        _ => "General",
    };

    cx.render(rsx! {
        div {
            id: "settings",
            class: "{sidebar_visibility}",
            sidebar::SettingsSidebar {
                account: cx.props.account.clone(),
                on_pressed: move |ne| {
                    active_page.set(ne);

                    let state = use_atom_ref(&cx, STATE).clone();
                    state.write().dispatch(Actions::HideSidebar(true));
                },
                initial_value: match active_page.get() {
                    Route::Profile => Route::Profile,
                    Route::Developer => Route::Developer,
                    _ => Route::General,
                },
            },
            div {
                id: "content",
                div {
                    class: "toolbar-wrapper",
                    toolbar::Toolbar {
                        controls: cx.render(rsx! {
                            div {}
                        }),
                        div {
                            class: "toolbar-content",
                            div {
                                class: "toolbar-start",
                                div {
                                    class: "mobile-back-button",
                                    IconButton {
                                        icon: Shape::ArrowLeft,
                                        state: ui_kit::icon_button::State::Secondary,
                                        on_pressed: move |_| {
                                            let state = use_atom_ref(&cx, STATE).clone();
                                            state.write().dispatch(Actions::HideSidebar(false));
                                        },
                                    },
                                },
                            },
                            h1 {
                                "{active_page_string}",
                            },
                            div {
                                class:  "toolbar-end",
                            }
                        }
                    },
                },
                div {
                    id: "page",
                    div {
                        class: "wrapper",
                        div {
                            class: "content",
                            match active_page.get() {
                                Route::General => rsx!(General { account: cx.props.account.clone() }),
                                Route::Developer => rsx!(Developer { account: cx.props.account.clone() }),
                                Route::Profile => rsx!(Profile { account: cx.props.account.clone() }),
                                Route::Extensions => rsx!(Extensions {}),
                                _ => rsx!(Developer { account: cx.props.account.clone() }),
                            }
                        }
                    }
                }
            }
        }
    })
}
