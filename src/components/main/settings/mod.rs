use dioxus::prelude::*;

use crate::{
    components::main::settings::pages::{
        developer::Developer, extensions::Extensions, general::General, profile::Profile,
    },
    Account,
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
    log::debug!("rendering Settings");
    let page_to_open_on_settings = match cx.props.page_to_open {
        Route::Profile => Route::Profile,
        Route::Developer => Route::Developer,
        _ => Route::General,
    };

    let active_page = use_state(&cx, || page_to_open_on_settings);

    cx.render(rsx! {
        div {
            id: "settings",
            sidebar::SettingsSidebar {
                account: cx.props.account.clone(),
                on_pressed: move |ne| {
                    active_page.set(ne);
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
