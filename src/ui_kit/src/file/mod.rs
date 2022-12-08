use std::{ffi::OsStr, path::PathBuf};

use dioxus::{core::to_owned, prelude::*};
use dioxus_elements::KeyCode;
use dioxus_heroicons::{outline::Shape, Icon};
use utils::Storage;

use super::folder::State;
use crate::context_menu::{ContextItem, ContextMenu};
use rfd::FileDialog;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    name: String,
    state: State,
    id: String,
    kind: String,
    size: usize,
    thumbnail: String,
    storage: Storage,
}

#[allow(non_snake_case)]
pub fn File(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let file_id = cx.props.id.clone();

    let file_name_fmt = format_file_name_to_show(cx.props.name.clone(), cx.props.kind.clone());

    let file_name_formatted_state = use_state(&cx, || file_name_fmt);

    let file_name_complete_ref = use_ref(&cx, || cx.props.name.clone());

    let file_size = format_file_size(cx.props.size);
    let file_thumb = &cx.props.thumbnail.clone();

    let show_edit_name_script = include_str!("./show_edit_name.js").replace("file_id", &file_id);
    let file_component =  
    if cx.props.thumbnail.is_empty() {
        rsx!(Icon { icon: Shape::Document })
    } else {
        rsx!(img {
            src: "{file_thumb}",
            width: "80%",
            height: "80%",
            border_radius: "8px",
            })
    };

    cx.render(rsx! {
        div {
            class: "item file",
            id: "{file_id}-file",
                ContextMenu {
                    parent: format!("{}-file", file_id.clone()),
                    items: cx.render(
                        rsx! {
                                ContextItem {
                                    icon: Shape::PencilSquare,
                                    onpressed: move |_| {
                                        //TODO(File): Investigate in a way to replace use_eval in the future
                                        // Use js script to show edit file name element
                                        use_eval(&cx)(&show_edit_name_script);
                                    },
                                    text: String::from("Rename")
                                },
                                ContextItem {
                                    icon: Shape::DocumentArrowDown,
                                    onpressed: move |_| {
                                        hide_edit_name_element(cx);
                                        let file_storage = cx.props.storage.clone();
                                        let file_name = &*file_name_complete_ref.read();
                                        let file_extension = cx.props.kind.clone();
                                        cx.spawn({
                                            to_owned![file_storage, file_name, file_extension];
                                            async move {
                                                let file_path_buff = match FileDialog::new().set_directory(".").set_file_name(&file_name).add_filter("", &[&file_extension]).save_file() {
                                                    Some(path) => path,
                                                    None => return,
                                                };
                                                let file_path_str = file_path_buff.to_string_lossy().to_string();
                                                match file_storage.get(&file_name, &file_path_str).await {
                                                    Ok(_) => log::info!("{file_name} downloaded."),
                                                    Err(error) => log::error!("Error downloading file {}: {error}", &file_name),
                                                };
                                            }
                                        });
                                    },
                                    text: String::from("Download")
                                },
                                hr {},
                                ContextItem {
                                    onpressed: move |_| {
                                        hide_edit_name_element(cx);
                                        let file_storage = cx.props.storage.clone();
                                        let file_name = &*file_name_complete_ref.read();
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
                    }),
                },
            div {
                rsx!(    
                    div {
                        class: "folder {class}",
                        div {
                            class: "thumb_icon",
                            file_component,
                        }
                        {
                        let val = use_ref(&cx, String::new);
                        let complete_file_name = file_name_complete_ref.read();
                        let file_id = file_id.clone();
                        rsx! {
                            p {
                                id: "{file_id}-name-normal",
                                "{file_name_formatted_state}" }
                            input {
                            id: "{file_id}-input",
                            display: "none",
                            class: "new_folder_input",
                            placeholder: "{complete_file_name}",
                            onchange: move |evt| {
                                *val.write_silent() = evt.value.to_string();
                            },
                            onkeyup: move |evt| {
                                if evt.key_code == KeyCode::Enter {
                                    let file_storage = cx.props.storage.clone();
                                    let old_file_name = &*file_name_complete_ref.read();
                                    let file_extension = cx.props.kind.clone();
                                    let new_file_name = val.read();
                                    hide_edit_name_element(cx);

                                    if !new_file_name.trim().is_empty() {
                                        cx.spawn({
                                            to_owned![file_storage, old_file_name, new_file_name, file_extension, file_name_formatted_state, file_name_complete_ref];
                                            async move {
                                                let new_file_name_with_extension = format_args!("{}.{}", new_file_name.trim(), file_extension.clone()).to_string();

                                                match file_storage.rename(&old_file_name, &new_file_name_with_extension).await {
                                                    Ok(_) => {
                                                    let new_file_name_fmt =
                                                        format_file_name_to_show(new_file_name_with_extension.clone(), file_extension);

                                                        *file_name_complete_ref.write_silent() = new_file_name_with_extension.clone();
                                                        file_name_formatted_state.set(new_file_name_fmt);


                                                        log::info!("{old_file_name} renamed to {new_file_name_with_extension}");
                                                    },
                                                    Err(error) => log::error!("Error renaming file: {error}"),
                                                };
                                            }
                                        });

                                    }

                                }
                            }
                        }
                        label {
                            "{file_size}"
                        }
                        }
                        }
                    }
                )}
        }
    })
}

fn hide_edit_name_element(cx: Scope<Props>) {
    //TODO(File): Investigate in a way to replace use_eval in the future
    // Use js script to hide edit file name element
    let hide_edit_name_script =
        include_str!("./hide_edit_name.js").replace("file_id", &cx.props.id.clone());
    use_eval(&cx)(&hide_edit_name_script);
}

fn format_file_size(file_size: usize) -> String {
    let base_1024: f64 = 1024.0;
    let size_f64: f64 = file_size as f64;

    let i = (size_f64.log10() / base_1024.log10()).floor();
    let size_formatted = size_f64 / base_1024.powf(i);

    let file_size_suffix = ["bytes", "KB", "MB", "GB", "TB"][i as usize];
    let mut size_formatted_string = format!(
        "{size:.*} {size_suffix}",
        1,
        size = size_formatted,
        size_suffix = file_size_suffix
    );
    if size_formatted_string.contains(".0") {
        size_formatted_string = size_formatted_string.replace(".0", "");
    }
    size_formatted_string
}

fn format_file_name_to_show(file_name: String, file_kind: String) -> String {
    let mut new_file_name = file_name.clone();
    let file = PathBuf::from(&new_file_name);
    let file_stem = file
        .file_stem()
        .and_then(OsStr::to_str)
        .map(str::to_string)
        .unwrap_or_default();

    if file_stem.len() > 10 {
        new_file_name = match &file_name.get(0..5) {
            Some(name_sliced) => format!(
                "{}...{}.{}",
                name_sliced,
                &file_stem[file_stem.len() - 3..].to_string(),
                file_kind
            ),
            None => file_name.clone(),
        };
    }
    new_file_name
}
