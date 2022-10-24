use dioxus::prelude::*;

pub mod browser;
pub mod sidebar;
pub mod toolbar;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
}

#[allow(non_snake_case)]
pub fn Files(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "files",
            sidebar::Sidebar { account: cx.props.account.clone() },
            div {
                id: "content",
                toolbar::Toolbar { account: cx.props.account.clone() },
                browser::FileBrowser { account: cx.props.account.clone() }
            },
        }
    })
}
