use std::{collections::HashSet, time::Duration};

use dioxus::prelude::*;

use crate::Storage;
use ui_kit::{file::File, folder::{State, Folder}, new_folder::NewFolder};
use warp::constellation::item::ItemType;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: Storage,
    show_new_folder: UseState<bool>,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {
    let file_storage = cx.props.storage.clone();
    let files = use_ref(&cx, HashSet::new);
    let files_sorted = use_state(&cx, Vec::new);

    use_future(
        &cx,
        (files, files_sorted, &file_storage.root_directory()),
        |(files, files_sorted, root_directory)| async move {
            loop {
                let files_updated: HashSet<_> = HashSet::from_iter(root_directory.get_items());

                if *files.read() != files_updated {
                    log::debug!("updating files list");
                    *files.write_silent() = files_updated.clone();
                    let mut total_files_list: Vec<_> = files_updated.iter().cloned().collect();
                    total_files_list.sort_by_key(|b| std::cmp::Reverse(b.modified()));
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
                div {
                    class: "item file",
                    NewFolder {
                        state: State::Primary,
                        storage: cx.props.storage.clone(),
                        show_new_folder: cx.props.show_new_folder.clone(),
                    }
                }
            )),
            files_sorted.iter().filter(|item| item.item_type() == ItemType::DirectoryItem).map(|directory| {
                let key = directory.id();

                rsx!(
                    Folder {
                        key: "{key}",
                        name: directory.name(),
                        state: State::Primary,
                        id: key.to_string(),
                        size: directory.size(),
                        children: 0,
                        storage: cx.props.storage.clone(),
                    }
        )
            })
            files_sorted.iter().filter(|item| item.item_type() == ItemType::FileItem).map(|file| {


                let file_extension = std::path::Path::new(&file.name())
                    .extension()
                    .unwrap_or_else(|| std::ffi::OsStr::new(""))
                    .to_str()
                    .unwrap()
                    .to_string();

                let key = file.id();

                rsx!(

                            File {
                                key: "{key}",
                                name: file.name(),
                                state: State::Secondary,
                                id: key.to_string(),
                                kind: file_extension,
                                size: file.size(),
                                thumbnail: file.thumbnail(),
                                storage: cx.props.storage.clone(),
                            }

                )
            })
        }
    })
}
