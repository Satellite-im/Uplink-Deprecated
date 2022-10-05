use dioxus::prelude::*;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>,
}

#[allow(non_snake_case)]
pub fn Loader(cx: Scope<Props>) -> Element {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("extra/uplink.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(4));

    cx.render(rsx! {
        div {
            class: "load",
            img {
                src: "extra/uplink.gif"
            }
            span {
                cx.props.text.clone()
            },
            div {
                class: "bar"
            }
        }
    })
}
