use dioxus::prelude::*;

pub mod browser;
pub mod sidebar;
pub mod toolbar;
pub mod upload;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: crate::Storage,
}

#[allow(non_snake_case)]
pub fn Files(cx: Scope<Props>) -> Element {
    let show_new_folder = use_state(&cx, || false);
    let show_upload = use_state(&cx, || false);

    cx.render(rsx! {
        div {
            id: "files",
            sidebar::Sidebar { account: cx.props.account.clone() },
            div {
                id: "content",
                toolbar::Toolbar {
                    on_new_folder: move |_| {
                        show_new_folder.set(true);
                    },
                    on_show_upload: move |_| {
                        show_upload.set(true);
                    }
                },
                upload::Upload {
                    show: **show_upload,
                    on_hide: move |_| show_upload.set(false),
                },
                browser::FileBrowser {
                    account: cx.props.account.clone(),
                    show_new_folder: **show_new_folder
                }
            },
        }
    })
}
