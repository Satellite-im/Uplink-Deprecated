use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use humansize::{format_size, DECIMAL};

use ui_kit::{
    button::{self, Button},
    icon_button::{self, IconButton},
    select::Select,
    switch::Switch,
};
use utils::extensions::{BasicExtension, Extension, ExtensionType};

#[derive(Props)]
pub struct OptionProps<'a> {
    title: String,
    text: String,
    icon: Shape,
    children: Element<'a>,
}

#[allow(non_snake_case)]
fn AudioOption<'a>(cx: Scope<'a, OptionProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "row option",
            div {
                class: "info",
                div {
                    class: "icon",
                    Icon {
                        icon: cx.props.icon
                    }
                }
                div {
                    class: "text",
                    label {
                        "{cx.props.title}"
                    },
                    p {
                        "{cx.props.text}"
                    },
                }
            },
            div {
                class: "control",
                &cx.props.children
            }
        }
    })
}

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    debug: bool,
}

#[allow(non_snake_case)]
pub fn ExtAudioFactory(cx: Scope<Props>) -> Element {
    let free_space = match fs2::free_space("/") {
        Ok(space) => space,
        Err(_) => 1,
    };
    let total_space = match fs2::total_space("/") {
        Ok(space) => space,
        Err(_) => 1,
    };
    let perc = (((total_space / free_space) as f64) * 0.1) * 100.0;
    let space = format!(
        "{}/{} Free",
        format_size(free_space, DECIMAL),
        format_size(total_space, DECIMAL)
    );

    let advanced_visible = use_state(&cx, || false);
    let multitrack = use_state(&cx, || false);
    let recording = use_state(&cx, || false);

    let main_class = if **recording {
        String::from("restricted")
    } else {
        String::from("can-modify")
    };

    let restricted_advanced_options = if **multitrack {
        String::from("restricted")
    } else {
        String::from("can-modify")
    };

    cx.render(rsx! {
        div {
            id: "audio-factory",
            div {
                class: "row",
                Button {
                    text: if **recording {String::from("⏹ Stop Recording")} else {String::from("⏺ Start Recording")},
                    state: if **recording {button::State::Danger} else {button::State::Secondary},
                    on_pressed: move |_| recording.set(!recording)
                },
            },
            div {
                class: "{main_class}",
                AudioOption {
                    title: String::from("Record Audio"),
                    text: String::from("Record call audio to disk?"),
                    icon: Shape::Microphone,
                    children: cx.render(rsx! {
                        Switch {
                            active: true,
                            on_change: move |_| {}
                        }
                    })
                },
                AudioOption {
                    title: String::from("Record Video"),
                    text: String::from("Record call video to disk?"),
                    icon: Shape::VideoCamera,
                    children: cx.render(rsx! {
                        Switch {
                            active: true,
                            on_change: move |_| {}
                        }
                    })
                },
                AudioOption {
                    title: String::from("Output Location"),
                    text: String::from("c://fake/path/changeme"),
                    icon: Shape::FolderArrowDown,
                    children: cx.render(rsx! {
                        Button {
                            text: String::from("Change"),
                            state: button::State::Primary,
                            on_pressed: move |_| {}
                        },
                    })
                },
                p {
                    class: "avail",
                    span {
                        class: "progress",
                        span {
                            class: "bar",
                            style: "width: {perc}%"
                        }
                    },
                    "Disk Space: {space}"
                },
                div {
                    class: "row",
                    Button {
                        text: if **advanced_visible {String::from("Hide Advanced")} else {String::from("Advanced Options")},
                        state: if **advanced_visible {button::State::Primary} else {button::State::Secondary},
                        on_pressed: move |_| advanced_visible.set(!advanced_visible)
                    },
                },
                (**advanced_visible).then(|| rsx! {
                    div {
                        class: "advanced-options",
                        AudioOption {
                            title: String::from("Multi-Track Recording"),
                            text: String::from("Merge all recordings into a multi-track mp4."),
                            icon: Shape::Cog,
                            children: cx.render(rsx! {
                                Switch {
                                    active: **multitrack,
                                    on_change: move |_| multitrack.set(!multitrack)
                                }
                            })
                        },
                        div {
                            class: "{restricted_advanced_options}",
                            AudioOption {
                                title: String::from("Video Format"),
                                text: String::from("Sets the outputted video file format."),
                                icon: Shape::Cog,
                                children: cx.render(rsx! {
                                    Select {
                                        on_change: move |_v| {},
                                        options: vec![String::from("MP4")]
                                    }
                                })
                            },
                        },
                        AudioOption {
                            title: String::from("Video Quality"),
                            text: String::from("Sets the quality and framerate of the video recording."),
                            icon: Shape::Cog,
                            children: cx.render(rsx! {
                                Select {
                                    on_change: move |_v| {},
                                    // TODO: Automate this
                                    options: vec![String::from("8K-120"), String::from("8K-60"), String::from("4K-120"), String::from("4K-60"), String::from("4K-30"), String::from("1080-120"), String::from("1080-60"), String::from("1080-30"), String::from("720-120"), String::from("720-60"), String::from("720-30"), String::from("360-120"), String::from("360-60"), String::from("360-30")]
                                }
                            })
                        },
                        div {
                            class: "{restricted_advanced_options}",
                            AudioOption {
                                title: String::from("Audio Format"),
                                text: String::from("Sets the outputted audio file format."),
                                icon: Shape::Cog,
                                children: cx.render(rsx! {
                                    Select {
                                        on_change: move |_v| {},
                                        options: vec![String::from("FFV1"), String::from("FAAC"), String::from("HEVC"), String::from("AAC"), String::from("Ape"), String::from("AIFF"), String::from("FLAC"), String::from("MP3"), String::from("MP4"), String::from("Opus"), String::from("Ogg Vorbis"), String::from("Speex"), String::from("Wav"), String::from("WavPack")]
                                    }
                                })
                            },
                        },
                        AudioOption {
                            title: String::from("Audio Quality"),
                            text: String::from("Sets the quality the audio recording."),
                            icon: Shape::Cog,
                            children: cx.render(rsx! {
                                Select {
                                    on_change: move |_v| {},
                                    options: vec![String::from("LOSSLESS")]
                                }
                            })
                        },
                        AudioOption {
                            title: String::from("Record Output"),
                            text: String::from("Records any sounds outputted to your headphones on their own track."),
                            icon: Shape::Cog,
                            children: cx.render(rsx! {
                                Switch {
                                    active: true,
                                    on_change: move |_| {}
                                }
                            })
                        },
                        AudioOption {
                            title: String::from("Record System Sounds"),
                            text: String::from("When enabled, notifications and other system sounds will be saved."),
                            icon: Shape::Cog,
                            children: cx.render(rsx! {
                                Switch {
                                    active: false,
                                    on_change: move |_| {}
                                }
                            })
                        },
                    }
                })
            }
        }
    })
}

pub struct AudioFactory;

impl BasicExtension for AudioFactory {
    fn info() -> Extension {
        Extension {
            name: String::from("Audio Factory"),
            author: String::from("matt@satellite.im"),
            description: String::from("Audio Factory allows you to record all in app audio to different audio tracks. You can additionally change the format, compress and convert."),
            location: ExtensionType::ChatbarIcon,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        // TODO: Icon should be a record icon, it should turn red and become a ovular shape like a normal button which includes the duration of the recording and turns the icon red
        let factory_visible = use_state(&cx, || false);

        cx.render(rsx! {
            div {
                id: "audio-factory",
                (factory_visible).then(|| rsx! {
                    ExtAudioFactory {
                        debug: false
                    }
                }),
                IconButton {
                    icon: Shape::Film,
                    state: if **factory_visible {
                        icon_button::State::Primary
                    } else {
                        icon_button::State::Secondary
                    }
                    on_pressed: move |_| factory_visible.set(!factory_visible)
                }
            }
        })
    }
}
