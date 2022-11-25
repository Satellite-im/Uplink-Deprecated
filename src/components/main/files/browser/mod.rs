use std::{collections::HashSet, time::Duration};

use dioxus::{core::to_owned, prelude::*};

use dioxus_heroicons::outline::Shape;
use ui_kit::context_menu::{ContextItem, ContextMenu};
use ui_kit::{file::File, folder::State, new_folder::NewFolder};
use warp::constellation::item::ItemType;
use warp::constellation::Constellation;

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
        (files, files_sorted, &file_storage.root_directory()),
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
                div {
                    class: "item file",
                    NewFolder {
                        state: State::Primary
                    }
                }
            )),
            files_sorted.iter().filter(|item| item.item_type() == ItemType::FileItem).map(|file| {

                let filname = file.name();
                let file_extension = std::path::Path::new(&file.name())
                    .extension()
                    .unwrap_or_else(|| std::ffi::OsStr::new(""))
                    .to_str()
                    .unwrap()
                    .to_string();

                rsx!(
                    div {
                        class: "item file",
                        id: "{filname}-file",
                        ContextMenu {
                            parent: format!("{}-file", filname),
                            items: cx.render(rsx! {
                                ContextItem {
                                    icon: Shape::PencilAlt,
                                    onpressed: move |_| {},
                                    text: String::from("Rename")
                                },
                                ContextItem {
                                    icon: Shape::DocumentDownload,
                                    onpressed: move |_| {
                                        // TODO(Files): Add download function here
                                        eprintln!("Download item");
                                    },
                                    text: String::from("Download")
                                },
                                hr {},
                                ContextItem {
                                    onpressed: move |_| {
                                        let file_storage = cx.props.storage.clone();
                                        let file_name = file.name();
                                        cx.spawn({
                                            to_owned![file_storage, file_name];
                                            async move {
                                                match file_storage.remove(&file_name, true).await {
                                                    Ok(_) => log::info!("{file_name} was deleted."),
                                                    Err(error) => log::error!("Error deleting file: {error}"),
                                                };
                                            }
                                        });
                                    },
                                    icon: Shape::Trash,
                                    danger: true,
                                    text: String::from("Delete")
                                },
                            })
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
            })
        }
    })
}
