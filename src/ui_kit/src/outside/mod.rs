use dioxus::prelude::*;
use warp::crypto::rand::{self, distributions::Alphanumeric, Rng};

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn OutsideClick<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let rand_s: &String = cx.use_hook(|_| {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect()
    });

    let script = include_str!("outside.js").replace("ID", rand_s);

    cx.render(rsx! {
        span {
            id: "outside-container-{rand_s}",
            &cx.props.children,
            script { "{script}" },
        }
    })
}
