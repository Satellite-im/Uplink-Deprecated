use std::fs::File;
use dioxus::core::to_owned;
use dioxus::desktop::use_window;
use dioxus::router::use_router;
use dioxus::prelude::*;
use rodio::{source::Source, Decoder, OutputStream};
use std::io::BufReader;
use futures::StreamExt;
use crate::{
    Account, WINDOW_SUFFIX_NAME,
};

// Remember: owned props must implement PartialEq!
#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Loading(cx: Scope<Props>) -> Element {
    let window = use_window(&cx);
    let loaded = use_state(&cx, || false);
    let tx: &CoroutineHandle<bool> = use_coroutine(&cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![loaded];
        async move {
            while let Some(flag) = rx.next().await {
                if flag {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    loaded.set(true);
                    break;
                }
            }
        }
    });
    std::thread::sleep(std::time::Duration::from_millis(10));
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("extra/assets/uplink.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    if let Err(_e) = stream_handle.play_raw(source.convert_samples()) {
        std::thread::sleep(std::time::Duration::from_secs(2));
        //TODO: Do something if it fails to load audio?
    }
    let multipass = cx.props.account.clone();
    let _account_fetch_status = match multipass.read().get_own_identity() {
        Ok(i) => {
            if *loaded.get() {
                window.set_title(&format!("{} - {}", i.username(), WINDOW_SUFFIX_NAME));
                use_router(&cx).push_route("/main", None, None);
            } else {
                tx.send(true);
            }
            false
        }
        Err(_) => {
            use_router(&cx).push_route("/auth", None, None);
            true
        },
    };

    cx.render(rsx! {
        img {
            style: "width: 100%",
            src: "extra/assets/uplink.gif"
        }
    })
}
