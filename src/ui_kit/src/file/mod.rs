use std::time::Duration;

use dioxus::{core::to_owned, prelude::*};
use dioxus_elements::KeyCode;
use dioxus_heroicons::{outline::Shape, Icon};
use utils::Storage;
use warp::constellation::Constellation;

use super::folder::State;
use super::icon_button::State as icon_state;
use crate::icon_button::IconButton;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    name: String,
    state: State,
    kind: String,
    size: usize,
    thumbnail: String,
    storage: Storage,
}

#[allow(non_snake_case)]
pub fn File<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let file_name_ref = use_ref(&cx, String::new);

    let file_name_formatted_state = use_state(&cx, String::new);

    let file_name_complete_state = use_state(&cx, || cx.props.name.clone());

    let start_edit_name = use_state(&cx, || false);

    let file_size = format_file_size(cx.props.size);    

    use_future(&cx, (file_name_ref, file_name_complete_state, file_name_formatted_state, &cx.props.name.clone(), &cx.props.kind.clone()), 
    |(file_name_ref, file_name_complete_state, file_name_formatted_state, file_name, file_extension)|
     async move {

        if *file_name_ref.read() != file_name {
            *file_name_ref.write_silent() = file_name.clone();
            let file_name_fmt = format_file_name_to_show(file_name.clone(), file_extension);
            file_name_complete_state.set(file_name);
            file_name_formatted_state.set(file_name_fmt);
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    });

    cx.render(rsx! {
            div {
                class: "dropdown",
            div {
                class: "item file",
            div {
                class: "folder {class}",
                    Icon { icon: Shape::Document},
                    if **start_edit_name {
                        let val = use_ref(&cx, String::new);
                        rsx!(
                            input {
                            class: "new_file_input",
                            autofocus: "true",
                            placeholder: "{file_name_complete_state}",
                            onchange: move |evt| {
                                *val.write_silent() = evt.value.to_string();
                            },
                            onkeyup: move |evt| {
                                if evt.key_code == KeyCode::Enter {
                                    start_edit_name.set(false);
                                    let file_storage = cx.props.storage.clone();
                                    let old_file_name = file_name_complete_state.clone();
                                    let file_extension = cx.props.kind.clone();
                                    let new_file_name = val.read();

                                    if !new_file_name.trim().is_empty() {
                                        cx.spawn({
                                            to_owned![file_storage, old_file_name, new_file_name, file_extension, file_name_formatted_state, file_name_complete_state];
                                            async move {
                                                let new_file_name_with_extension = format_args!("{}.{}", new_file_name.trim(), file_extension.clone()).to_string();

                                                match file_storage.rename(&old_file_name, &new_file_name_with_extension).await {
                                                    Ok(_) => {
                                                    let new_file_name_fmt =
                                                        format_file_name_to_show(new_file_name_with_extension.clone(), file_extension);

                                                        file_name_formatted_state.set(new_file_name_fmt);
                                                        file_name_complete_state.set(new_file_name_with_extension.clone());
    
                                                    log::info!("{old_file_name} renamed to {new_file_name_with_extension}");
                                                    },
                                                    Err(error) => log::error!("Error renaming file: {error}"),
                                                };
                                            }
                                        });
                                    }

                                    
                                }
                            }
                        })
                    } else {
                        rsx!(p { "{file_name_formatted_state}" })
                }
                    label {
                        "{file_size}"
                    }
            }
        }
        div {
            class: "dropdown-content",
            IconButton {
                icon: Shape::X,
                state: icon_state::Secondary,
                on_pressed: move |_| {
                    let file_storage = cx.props.storage.clone();
                    let file_name = file_name_complete_state.clone();
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
            }
            IconButton {
                icon: Shape::Download,
                state: icon_state::Secondary,
                on_pressed: move |_| {
                    // TODO(Files): Add download function here
                    eprintln!("Download item");
                },
            }
            IconButton {
                icon: Shape::Pencil,
                state: icon_state::Secondary,
                on_pressed: move |_| {
                    start_edit_name.set(!start_edit_name);
                },
            }
        }
    }

        })
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

    let file_name_without_extension = std::path::Path::new(&file_name)
        .with_extension("")
        .to_str()
        .unwrap()
        .to_string();

    if file_name_without_extension.len() > 10 {
        new_file_name = match &file_name.get(0..5) {
            Some(name_sliced) => format!(
                "{}...{}.{}",
                name_sliced,
                &file_name_without_extension[file_name_without_extension.len() - 3..].to_string(),
                file_kind
            ),
            None => file_name.clone(),
        };
    }
    new_file_name
}
