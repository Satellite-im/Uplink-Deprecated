use dioxus::desktop::use_window;
use dioxus::router::use_router;
use dioxus::prelude::*;

use crate::{
    components::ui_kit::{
        loader::Loader,
    },
    Account, LANGUAGE, WINDOW_SUFFIX_NAME,
};

// Remember: owned props must implement PartialEq!
#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Loading(cx: Scope<Props>) -> Element {
    let window = use_window(&cx);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    std::thread::sleep(std::time::Duration::from_millis(10));

    let multipass = cx.props.account.clone();
    let _account_fetch_status = match multipass.read().get_own_identity() {
        Ok(i) => {
            window.set_title(&format!("{} - {}", i.username(), WINDOW_SUFFIX_NAME));
            use_router(&cx).push_route("/main", None, None);
            false
        }
        Err(_) => {
            use_router(&cx).push_route("/auth", None, None);
            true
        },
    };

    cx.render(rsx! {
        Loader {
            text: l.checking_account.clone()
        }
    })
}
