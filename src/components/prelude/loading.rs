use crate::{iutils::config::Config, Account, LANGUAGE, WINDOW_SUFFIX_NAME};
use dioxus::core::to_owned;
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use dioxus::router::use_router;
use futures::StreamExt;
use ui_kit::loader::Loader;

// Remember: owned props must implement PartialEq!
#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Loading(cx: Scope<Props>) -> Element {
    log::debug!("rendering Loading");
    let config = Config::load_config_or_default();
    let window = use_window(&cx);
    let loaded = use_state(&cx, || false);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let tx: &CoroutineHandle<bool> = use_coroutine(&cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![loaded];
        async move {
            while let Some(flag) = rx.next().await {
                if flag {
                    loaded.set(true);
                    break;
                }
            }
        }
    });
    std::thread::sleep(std::time::Duration::from_millis(10));
    let multipass = cx.props.account.clone();
    let _account_fetch_status = match multipass.read().get_own_identity() {
        Ok(i) => {
            if *loaded.get() {
                // if config.general.show_splash {
                //     // Get a output stream handle to the default physical sound device
                //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                //     // Load a sound from a file, using a path relative to Cargo.toml
                //     let file = BufReader::new(File::open("extra/assets/uplink.mp3").unwrap());
                //     // Decode that sound file into a source
                //     let source = Decoder::new(file).unwrap();
                //     // Play the sound directly on the device
                //     let _ = stream_handle.play_raw(source.convert_samples());
                //     std::thread::sleep(std::time::Duration::from_secs(2));
                // }
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
        }
    };

    cx.render(if config.general.show_splash {
        rsx! {
            img {
                style: "width: 100%",
                src: "extra/assets/img/uplink.gif"
            }
        }
    } else {
        rsx! {
            Loader {
                text: l.checking_account.clone()
            }
        }
    })
}
