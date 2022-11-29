use crate::Messaging;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::Icon;
use dioxus_html::on::MouseEvent;
use futures::StreamExt;
use humansize::format_size;
use humansize::DECIMAL;
use rfd::FileDialog;
use ui_kit::button;
use warp::constellation::file::File;
use warp::raygun::Message;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    file: File,
    message: Message,
}

#[allow(non_snake_case)]
pub fn Attachment(cx: Scope<Props>) -> Element {
    let size = format_size(cx.props.file.size(), DECIMAL);
    let name = cx.props.file.name();

    let rg = cx.consume_context::<Messaging>().unwrap();

    let handle_click = move |evt: MouseEvent| {
        evt.cancel_bubble();

        cx.spawn_forever({
            let rg = rg.to_owned();
            let save_path = FileDialog::new()
                .set_file_name(&cx.props.file.name())
                .set_directory("/")
                .save_file();

            let conversation_id = cx.props.message.conversation_id();
            let id = cx.props.message.id();
            let name = cx.props.file.name();

            async move {
                match save_path {
                    Some(path) => match rg.download(conversation_id, id, name, path).await {
                        Ok(mut stream) => {
                            while let Some(progress) = stream.next().await {
                                match progress {
                                    warp::constellation::Progression::CurrentProgress {
                                        name,
                                        current,
                                        total,
                                    } => {
                                        // println!("Written {} MB for {name}", current / 1024 / 1024);
                                        if let Some(total) = total {
                                            println!(
                                                "{}% completed",
                                                (((current as f64) / (total as f64)) * 100.)
                                                    as usize
                                            )
                                        }
                                    }
                                    warp::constellation::Progression::ProgressComplete {
                                        name,
                                        total,
                                    } => {
                                        // TODO: Actual UI upload progress bindings.
                                        println!(
                                            "{name} downloaded with {} bytes written",
                                            total.unwrap_or_default()
                                        );
                                    }
                                    // TODO: Actual UI upload progress bindings.
                                    warp::constellation::Progression::ProgressFailed {
                                        name,
                                        error,
                                        ..
                                    } => {
                                        println!("{name} failed to download with error: {error:?}")
                                    }
                                }
                            }
                        }
                        Err(e) => println!("Error: {:?}", e),
                    },
                    None => {
                        println!("Path not provided");
                    }
                }
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "attachment-embed",
            div {
                class: "embed-icon",
                Icon {
                    icon: Shape::Document,
                },
                h2 {
                    "{name}"
                }
            }
            div {
                class: "embed-details",
                p {
                    "{size}"
                },
                button::Button {
                    icon: Shape::DocumentArrowDown,
                    text: String::from("Download File"),
                    state: button::State::Secondary,
                    on_pressed: handle_click
                }
            }
        }
    })
}
