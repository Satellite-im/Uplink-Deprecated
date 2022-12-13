use std::{collections::HashSet, time::Duration};

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::Storage;
use ui_kit::{file::File, folder::{State, Folder}, new_folder::NewFolder, button::Button};
use warp::constellation::{item::{ItemType}, directory::Directory};

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
    storage: Storage,
    show_new_folder: UseState<bool>,
    parent_directory: UseRef<Directory>,
}

#[allow(non_snake_case)]
pub fn FileBrowser(cx: Scope<Props>) -> Element {

    let files = use_ref(&cx, HashSet::new);
    let files_sorted = use_state(&cx, Vec::new);
    let parendt_directory_ref = cx.props.parent_directory.clone();
    let current_directory = cx.props.storage.current_directory().unwrap_or_default();

    use_future(
        &cx,
        (files, files_sorted, &current_directory),
        |(files, files_sorted, current_directory)| async move {

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

    let parent_directory_name =  cx.props.parent_directory.clone().read().name();
    cx.render(rsx! {
        div {
            flex_direction: "row",
            div {
                display: "inline-block",
                Button {
                    icon: Shape::Home,
                    on_pressed: move |_| {
                        let mut file_storage = cx.props.storage.clone();
                        loop {
                            let current_dir = file_storage.current_directory().unwrap();
                            if  current_dir.name() == "root" {
                                parendt_directory_ref.with_mut(|dir| *dir = current_dir.clone());
                                break;
                            }
                            file_storage.go_back().unwrap();
                        }
                    },
                },
            }
          
            h5 {
                display: "inline-block",
                margin_left: "8px",
                "{parent_directory_name}"},
        }
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
                    rsx!{
                         div {
                            key: "{key}-placeholder",
                        }
                        Folder {
                            key: "{key}"
                            name: directory.name(),
                            state: State::Primary,
                            id: key.to_string(),
                            size: directory.size(),
                            storage: cx.props.storage.clone(),
                            parent_directory:  cx.props.parent_directory.clone(),
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
                            parent_directory:  cx.props.parent_directory.clone(),
                        } 
                    }
                   )
            })
        }
    })
}
