use std::{collections::HashSet, time::Duration};

use dioxus::{core::to_owned, prelude::*};

use ui_kit::{file::File, folder::State, new_folder::NewFolder};
use warp::constellation::item::ItemType;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: crate::Storage,
    show_new_folder: bool,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {
    let file_storage = cx.props.storage.clone();
    let files = use_ref(&cx, HashSet::new);
    let files_sorted = use_state(&cx, Vec::new);

    use_future(
        &cx,
        (files, files_sorted, &file_storage.read().root_directory()),
        |(files, files_sorted, root_directory)| async move {
            loop {
                let files_updated: HashSet<_> = HashSet::from_iter(root_directory.get_items());

                if *files.read() != files_updated {
                    log::debug!("updating files list");
                    *files.write_silent() = files_updated.clone();

                    let mut total_files_list: Vec<_> = files_updated.iter().cloned().collect();

                    total_files_list.sort_by(|a, b| b.modified().cmp(&a.modified()));

                    files_sorted.set(total_files_list);
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        },
    );

    cx.render(rsx! {
        div {
            id: "browser",
            (cx.props.show_new_folder).then(|| rsx!(
                button {
                    class: "button-files file",
                    onclick: move |_| {},
                    NewFolder {
                        state: State::Primary
                    }
                }
            )),
            files_sorted.iter().filter(|item| item.item_type() == ItemType::FileItem).map(|file| {

                let file_extension = std::path::Path::new(&file.name())
                .extension()
                .unwrap_or_else(|| std::ffi::OsStr::new(""))
                .to_str()
                .unwrap()
                .to_string();

                rsx!(
                    button {
                        class: "button-files file",
                        onclick: move |_| {
                            let file_storage = cx.props.storage.clone();
                            let file_name = file.name();
                            cx.spawn({
                                to_owned![file_storage, file_name];
                                async move {
                                let mut write_storage = file_storage.write();

                                    match write_storage.remove(&file_name, true).await {
                                        Ok(_) => eprintln!("{file_name} was deleted."),
                                        Err(error) => eprintln!("Error deleting file: {error}"),
                                    };
                                }
                            });
                        },
                        File {
                            name: file.name(),
                            state: State::Secondary,
                            kind: file_extension,
                            size: file.size(),
                            thumbnail: file.thumbnail(),
                        }
                    }
                )
            }),
        },
    })
}
