use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use humansize::{format_size, DECIMAL};
use sir::css;

use ui_kit::{
    button::{self, Button},
    select::*,
    switch::Switch,
};
use utils::extensions::{BasicExtension, ExtensionInfo, ExtensionType};

#[derive(Props)]
pub struct OptionProps<'a> {
    title: String,
    text: String,
    icon: Shape,
    children: Element<'a>,
}

#[allow(non_snake_case)]
fn AudioOption<'a>(cx: Scope<'a, OptionProps<'a>>) -> Element<'a> {
    let styles = css!(
        "
        display: inline-flex;
        flex-wrap: wrap;
        border: 1px solid var(--theme-borders);
        border-radius: 4px;
        margin-bottom: 0.5rem;
        padding: 0.5rem 1rem;
        width: 100%;
        justify-content: center;

        & {
            .info {
                display: inline-flex;
                flex: 1;
                min-width: 200px;
                text-align: left;
                align-items: center;
                .icon {
                    display: inline-flex;
                    align-items: center;
                    margin-right: 1rem;
                    svg {
                        width: 30px;
                        height: 30px;
                    }
                }
            }

            .control {
                margin: 1rem 0;
                display: inline-flex;
                align-items: center;
                width: 20%;
                min-width: 200px;
                justify-content: center;
                .switch {
                    margin: 0 !important;
                }
            }
        }
        "
    );

    cx.render(rsx! {
        div {
            class: "row {styles}",
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
    let styles = css!(
        "
        border: 1px solid var(--theme-borders);
        background: var(--theme-background);
        padding: 1rem;
        position: absolute;
        z-index: 5;
        border-radius: 8px;
        left: 1rem;
        right: 1rem;
        bottom: 80px;
        max-height: 60vh;
        overflow-y: scroll;
        @media only screen and (min-width: 900px) {
            left: 50%;
        }
        &:hover {
            &::-webkit-scrollbar-thumb {
                background: var(--theme-primary) !important;
                opacity: 1;
            }
        }

        .button {
            width: 100%;
            margin: 0 0 0.5rem 0 !important;
        }

        .avail {
            padding: 0.5rem 1rem;
            text-align: center;
            border: 1px solid var(--theme-borders);
            border-radius: 4px;
            margin: 0.5rem 0;
            color: var(--theme-text-muted);
            font-size: var(--theme-text-small);
        }

        .restricted {
            cursor: not-allowed;

            div {
                opacity: 0.65;
                pointer-events: none;
            }
        }

        .progress {
            height: 0.5rem;
            border-radius: 0.25rem 0 0.25 0;
            background: var(--theme-background-light);
            overflow: hidden;
            width: 100%;
            display: inline-block;
            .bar {
                height: 100%;
                background: var(--theme-primary);
            }
        }
        "
    );

    let free_space = fs2::free_space("/").unwrap_or(1);
    let total_space = fs2::total_space("/").unwrap_or(1);
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
            class: "{styles}",
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
                    icon: Shape::ArrowDownOnSquare,
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
                                        value: String::from("mp4"),
                                        options: vec![
                                            SelectOption { label: String::from("MP4"), value: String::from("mp4") },
                                        ]
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
                                    value: String::from("720-30"),
                                    // TODO: Automate this
                                    options: vec![
                                        SelectOption { label: String::from("8K-120"), value: String::from("8K-120") },
                                        SelectOption { label: String::from("8K-60"), value: String::from("8K-60") },
                                        SelectOption { label: String::from("4K-120"), value: String::from("4K-120") },
                                        SelectOption { label: String::from("4K-60"), value: String::from("4K-60") },
                                        SelectOption { label: String::from("4K-30"), value: String::from("4K-30") },
                                        SelectOption { label: String::from("1080-120"), value: String::from("1080-120") },
                                        SelectOption { label: String::from("1080-60"), value: String::from("1080-60") },
                                        SelectOption { label: String::from("1080-30"), value: String::from("1080-30") },
                                        SelectOption { label: String::from("720-120"), value: String::from("720-120") },
                                        SelectOption { label: String::from("720-60"), value: String::from("720-60") },
                                        SelectOption { label: String::from("720-30"), value: String::from("720-30") },
                                        SelectOption { label: String::from("360-120"), value: String::from("360-120") },
                                        SelectOption { label: String::from("360-60"), value: String::from("360-60") },
                                        SelectOption { label: String::from("360-30"), value: String::from("360-30") }
                                    ]
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
                                        value: String::from("mp3"),
                                        options: vec![
                                            SelectOption { label: String::from("FFV1"), value: String::from("ffv1") },
                                            SelectOption { label: String::from("FAAC"), value: String::from("faac") },
                                            SelectOption { label: String::from("HEVC"), value: String::from("hevc") },
                                            SelectOption { label: String::from("AAC"), value: String::from("aac") },
                                            SelectOption { label: String::from("Ape"), value: String::from("ape") },
                                            SelectOption { label: String::from("AIFF"), value: String::from("aiff") },
                                            SelectOption { label: String::from("FLAC"), value: String::from("flac") },
                                            SelectOption { label: String::from("MP3"), value: String::from("mp3") },
                                            SelectOption { label: String::from("MP4"), value: String::from("mp4") },
                                            SelectOption { label: String::from("Opus"), value: String::from("opus") },
                                            SelectOption { label: String::from("Ogg Vorbis"), value: String::from("ogg vorbis") },
                                            SelectOption { label: String::from("Speex"), value: String::from("speex") },
                                            SelectOption { label: String::from("Wav"), value: String::from("wav") },
                                            SelectOption { label: String::from("WavPack"), value: String::from("wavpack") }
                                        ]
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
                                    value: String::from("lossless"),
                                    options: vec![
                                        SelectOption { label: String::from("LOSSLESS"), value: String::from("lossless") }
                                    ]
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
    fn info() -> ExtensionInfo {
        ExtensionInfo {
            name: String::from("Audio Factory"),
            author: String::from("matt@satellite.im"),
            description: String::from("Audio Factory allows you to record all in app audio to different audio tracks. You can additionally change the format, compress and convert."),
            location: ExtensionType::ChatbarIcon,
        }
    }

    fn render(cx: Scope) -> dioxus::prelude::Element {
        let styles = css!(
            "
            
            "
        );

        // TODO: Icon should be a record icon, it should turn red and become a ovular shape like a normal button which includes the duration of the recording and turns the icon red
        let factory_visible = use_state(&cx, || false);

        cx.render(rsx! {
            div {
                id: "audio-factory",
                class: "{styles}",
                (factory_visible).then(|| rsx! {
                    ExtAudioFactory {
                        debug: false
                    }
                }),
                Button {
                    icon: Shape::ViewfinderCircle,
                    state: if **factory_visible {
                        button::State::Primary
                    } else {
                        button::State::Secondary
                    }
                    on_pressed: move |_| factory_visible.set(!factory_visible)
                }
            }
        })
    }
}
