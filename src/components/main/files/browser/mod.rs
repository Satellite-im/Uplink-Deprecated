use std::{collections::HashSet, time::Duration};

use dioxus::prelude::*;

use ui_kit::{file::File, folder::State, new_folder::NewFolder};

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: crate::Storage,
    show_new_folder: bool,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {
    let file_storage = cx.props.storage.clone();
    let root_directory = &file_storage.read().root_directory();
    let files = use_state(&cx, || HashSet::from_iter(root_directory.get_items()));

    use_future(
        &cx,
        (files, &file_storage.read().root_directory()),
        |(files, root_directory)| async move {
            loop {
                let files_updated: HashSet<_> = HashSet::from_iter(root_directory.get_items());

                if *files != files_updated {
                    log::debug!("updating files list");
                    files.set(files_updated);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        },
    );

    cx.render(rsx! {
        div {
            id: "browser",
            (cx.props.show_new_folder).then(|| rsx!(
                NewFolder {
                    state: State::Primary
                }
            )),
            files.iter().map(|file| {
                let file_extension = std::path::Path::new(&file.name())
                .extension()
                .unwrap_or_else(|| std::ffi::OsStr::new(""))
                .to_str()
                .unwrap()
                .to_string();

                rsx!( File {
                    name: file.name(),
                    state: State::Secondary,
                    kind: file_extension,
                    size: file.size(),
                })
            }),
        },
    })
}
