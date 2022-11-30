use dioxus::prelude::*;

use crate::{
    components::main::settings::pages::{
        audio_video::AudioVideo, developer::Developer, extensions::Extensions, general::General,
        profile::Profile,
    },
    components::reusable::page_header,
    state::Actions,
    Account, STATE,
};

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
        false => "mobile-sidebar-visible",
        true => "mobile-sidebar-hidden",
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
                page_header::PageHeader {
                    content_start: cx.render(rsx! {Fragment()}),
                    content_center: cx.render(rsx! {
                        h1 { "{active_page_string}" }
                    }),
                    content_end: cx.render(rsx! {Fragment()}),
                    hide_on_desktop: true,
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
                                Route::AudioVideo => rsx!(AudioVideo {}),
                                _ => rsx!(Developer { account: cx.props.account.clone() }),
                            }
                        }
                    }
                }
            }
        }
    })
}
