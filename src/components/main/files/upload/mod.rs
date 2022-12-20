use std::{
    cmp::Ordering,
};

use dioxus::{
    core::to_owned,
    desktop::{use_window, wry::webview::FileDropEvent},
    events::MouseEvent,
    prelude::*,
};
use dioxus_heroicons::outline::Shape;

use futures::StreamExt;
use ui_kit::button::Button;
use rfd::FileDialog;
use crate::{DRAG_FILE_EVENT};
use utils::files_functions;

#[derive(Props)]
pub struct Props<'a> {
    storage: crate::Storage,
    show: bool,
    on_hide: EventHandler<'a, MouseEvent>,
}

enum Action {
    Start,
    Stop,
}

#[allow(non_snake_case)]
pub fn Upload<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let file_storage = cx.props.storage.clone();
    let drag_over_dropzone = use_ref(&cx, || false);
    let eval_script = use_window(&cx).clone();
    let file_over_dropzone_js = include_str!("./file_over_dropzone.js");
    let file_leave_dropzone_js = include_str!("./file_leave_dropzone.js");

    let upload_file_dropped_routine = use_coroutine(&cx, |mut rx: UnboundedReceiver<Action>| {
        to_owned![file_storage, drag_over_dropzone, eval_script, file_leave_dropzone_js, file_over_dropzone_js];
        async move {
            while let Some(action) = rx.next().await {
                match action {
                    Action::Start => {
                        log::info!("File on dropzone");
                        // Time necessary to work on macOS and Linux
                        #[cfg(not(target_os = "windows"))]
                        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                        if *drag_over_dropzone.read() {
                            let drag_file_event = get_drag_file_event();
                            let files_local_path = match drag_file_event.clone() {
                                FileDropEvent::Hovered(files_path)
                                | FileDropEvent::Dropped(files_path) => files_path,
                                _ => Vec::new(),
                            };

                            // TODO(use_eval): Try new solution in the future
                            match files_local_path.len().cmp(&1) {
                                Ordering::Greater => {
                                    let files_to_upload =
                                        format!("{} files to upload!", files_local_path.len());
                                    eval_script.eval(
                                        &file_over_dropzone_js
                                            .replace("file_path", &files_to_upload),
                                    );
                                }
                                Ordering::Equal => {
                                    eval_script.eval(&file_over_dropzone_js.replace(
                                        "file_path",
                                        &files_local_path[0].to_string_lossy(),
                                    ));
                                }
                                _ => ()
                            }

                            if let FileDropEvent::Dropped(files_local_path) = drag_file_event {
                                *drag_over_dropzone.write_silent() = false;
                                for file_path in &files_local_path {
                                    files_functions::upload_file(
                                        file_storage.clone(),
                                        file_path.clone(),
                                        eval_script.clone(),
                                    )
                                    .await;
                                    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                                    log::info!("{} file uploaded!", file_path.to_string_lossy());
                                }
                                // TODO(use_eval): Try new solution in the future
                                eval_script.eval(&file_leave_dropzone_js);
                            }
                        }
                    }
                    Action::Stop => {
                        eval_script.eval(&file_leave_dropzone_js);
                        log::info!("File not able to upload");
                        // HACK(Temp): Just to improve a little feedback for user on windows
                        // TODO(Temp): Temp solution to drag and drop work on Windows
                        #[cfg(target_os = "windows")]
                        loop {
                            if *drag_over_dropzone.read() {
                                break;
                            }
                            let drag_file_event = get_drag_file_event();
                            match drag_file_event {
                                FileDropEvent::Hovered(files_path) => {
                                    if files_path.len() > 1 {
                                        let files_to_upload = format!(
                                            "Dragging {} files. Drop here to upload them!",
                                            files_path.len()
                                        );
                                        eval_script.eval(
                                            &file_over_dropzone_js
                                                .replace("file_path", &files_to_upload),
                                        );
                                    } else if files_path.len() == 1 {
                                        eval_script.eval(&file_over_dropzone_js.replace(
                                            "file_path",
                                            "Dragging 1 file. Drop here to upload it!",
                                        ));
                                    }
                                }
                                _ => eval_script.eval(&file_leave_dropzone_js),
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        (cx.props.show).then(|| rsx! (
            div {
                id: "upload",
                onmouseover: move |_| {
                    // HACK(Windows): Not allow upload if drop file outside dropzone
                    // TODO(Temp): Temp solution to drag and drop work on Windows
                    #[cfg(target_os = "windows")]
                    if *drag_over_dropzone.read() == false {
                        *DRAG_FILE_EVENT.write() = FileDropEvent::Cancelled;
                    }
                },
                onmouseout:  move |_| {
                    *drag_over_dropzone.write_silent() = false;
                    upload_file_dropped_routine.send(Action::Stop);
                },
                div {
                    id: "content",
                    div {
                        width: "100%",
                        input {
                            "type": "file",
                            prevent_default: "onclick",
                            onclick: move |_| {
                                let files_local_path = match FileDialog::new().set_directory(".").pick_files() {
                                    Some(path) => path,
                                    None => return
                                };
                                let file_storage = cx.props.storage.clone();
                                cx.spawn({
                                    to_owned![file_storage, files_local_path, eval_script];
                                    async move {
                                        for file_path in &files_local_path {
                                            files_functions::upload_file(file_storage.clone(), file_path.clone(), 
                                            eval_script.clone(),
                                        ).await;
                                        }
                                    }
                                });
                            }
                        }
                        hr {
                           class: "hr-between-input-and-dropzone",
                        }
                        input {
                            id: "dropzone",
                            readonly: "true",
                            class: "dropzone",
                            value: "Drop files here to upload",
                            prevent_default: "onmouseover",
                            onmouseover: move |_| {
                                // HACK(Windows): When drop over dropzone, onmouseover is pushed
                                // TODO(Temp): Temp solution to drag and drop work on Windows
                                #[cfg(target_os = "windows")]
                                {
                                let drag_file_event = get_drag_file_event();
                                match drag_file_event {
                                    FileDropEvent::Dropped(_) => {
                                        *drag_over_dropzone.write_silent() = true;
                                        upload_file_dropped_routine.send(Action::Start);
                                    },
                                    _ => {
                                        *drag_over_dropzone.write_silent() = false;
                                        upload_file_dropped_routine.send(Action::Stop);
                                    }
                                }
                                }
                            },
                            onmouseout: move |_| {
                                *drag_over_dropzone.write_silent() = false;
                                upload_file_dropped_routine.send(Action::Stop);
                            },
                            ondragover: move |_| {
                                upload_file_dropped_routine.send(Action::Start);
                            },
                            ondragenter: move |_| {
                                *drag_over_dropzone.write_silent() = true;
                                // TODO(use_eval): Try new solution in the future
                                use_eval(&cx)(&file_over_dropzone_js.replace("file_path", ""));
                            },
                            ondragleave: move |_| {
                                *drag_over_dropzone.write_silent() = false;
                                // TODO(use_eval): Try new solution in the future
                                use_eval(&cx)(&file_leave_dropzone_js);
                                upload_file_dropped_routine.send(Action::Stop);
                            },
                        }
                    }
                },
                div {
                    id: "close",
                    Button {
                        on_pressed: move |e| {
                            cx.props.on_hide.call(e);
                        },
                        state: ui_kit::button::State::Secondary,
                        icon: Shape::XMark
                    }
                }
            }
        ))
    })
}

fn get_drag_file_event() -> FileDropEvent {
    let drag_file_event = DRAG_FILE_EVENT.read().clone();
    drag_file_event
}