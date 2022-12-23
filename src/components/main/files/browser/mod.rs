use std::{collections::HashSet, time::Duration, path::{PathBuf}};

use dioxus::prelude::*;
use dioxus_heroicons::{Icon, outline::Shape};
use utils::files_functions;

use crate::Storage;
use ui_kit::{file::File, folder::{State, Folder}, new_folder::NewFolder};
use warp::constellation::{item::{ItemType}};
mod lib;

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: Storage,
    show_new_folder: UseState<bool>,
    show_upload: UseState<bool>,
    dir_paths: UseRef<Vec<PathBuf>>,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {

    let files = use_ref(&cx, HashSet::new);
    let files_sorted = use_state(&cx, Vec::new);
    let root_directory = cx.props.storage.root_directory();
    let current_directory = cx.props.storage.current_directory().unwrap_or_else(|_| root_directory.clone());
    let update_current_dir = use_state(&cx, || ());
    let dir_paths = cx.props.dir_paths.clone();

    use_future(
        &cx,
        (files, files_sorted, &current_directory, &cx.props.storage.clone(), &cx.props.dir_paths.clone(), &cx.props.show_upload.clone(), &cx.props.show_new_folder.clone()),
        |(files, files_sorted, current_directory, files_storage, dir_paths, show_upload, show_new_folder)| async move {
           
            let current_dir_path = files_storage.get_path().clone();
            let dir_paths_vec = dir_paths.with(|vec| vec.clone());
            let dir_paths_len = dir_paths.read().len();
            let final_dir_path = dir_paths.read().last().unwrap().clone();

            if !dir_paths_vec.contains(&current_dir_path) {
                dir_paths.write().insert(dir_paths_len, current_dir_path);
                show_upload.set(false);
                show_new_folder.set(false);
            } else if final_dir_path != current_dir_path {
                dir_paths.write().remove(dir_paths_len - 1);
                show_upload.set(false);
                show_new_folder.set(false);
            } 
            

            loop {
                let files_updated: HashSet<_> = HashSet::from_iter(current_directory.get_items());
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
    let root_dir_id = root_directory.id();
    let current_dir_items_len = current_directory.get_items().len();
    let current_dir_size = files_functions::format_item_size(current_directory.size());

    cx.render(rsx! {
        div {
            dir_paths.read().iter().map(|current_dir_path| {
                if current_dir_path.to_string_lossy().to_string().is_empty() {
                    rsx!(
                        div {
                            class: "dir_paths_navigation",
                            margin_left: "8px",
                            padding_top: "4px",
                            display: "inline-block",
                            onclick: move |_| lib::go_back_dirs_with_loop(cx, root_dir_id),
                            Icon {
                                icon: Shape::Home
                            }
                        }
                    )
                } else {
                match root_directory.get_item_by_path(current_dir_path.to_str().unwrap_or_default())
                    .and_then(|item| item.get_directory()) {
                    Ok(directory) => {
                        let dir_name = directory.name();
                        let dir_id = directory.id();
                            rsx! (
                                h5 {
                                    margin_left: "8px",
                                    display: "inline-block",
                                    ">"},
                                h5 {
                                class: "dir_paths_navigation",
                                margin_left: "8px",
                                display: "inline-block",
                                onclick: move |_| lib::go_back_dirs_with_loop(cx, dir_id),
                            "{dir_name}"
                            })
                    },       
                _ =>  rsx!(div{}),
                }
            }
            })
        }
        label {
            margin_left: "8px",
            "{current_dir_size} / {current_dir_items_len} item(s)"
            },
        div {
         id: "browser",
            (cx.props.show_new_folder).then(|| 
                rsx!(
                    
                div {
                    class: "item file",
                    NewFolder {
                        state: State::Primary,
                        storage: cx.props.storage.clone(),
                        show_new_folder: cx.props.show_new_folder.clone(),
                    }
                }
            )
            ),
            files_sorted.iter().filter(|item| item.item_type() == ItemType::DirectoryItem).map(|directory| {
                let key = directory.id();
                let (dir_items_len, dir_size) =  if let Ok(dir) = directory.get_directory() {
                    (dir.get_items().len(), dir.size())
                } else {
                    (0, 0)
                };
                    rsx!{
                         div {
                            key: "{key}-placeholder",
                        }
                        Folder {
                            key: "{key}"
                            name: directory.name(),
                            state: State::Primary,
                            id: key.to_string(),
                            size: dir_size,
                            children: dir_items_len,
                            storage: cx.props.storage.clone(),
                            update_current_dir: update_current_dir.clone(),
                        }}
               
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
                    div {
                        onclick: move |_| {
                            if *cx.props.show_new_folder {
                                cx.props.show_new_folder.set(false);
                            }
                        },
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
                    }
                   )
            })
        }
    })
}