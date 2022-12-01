use dioxus::{core::to_owned, prelude::*};
use tokio::time::{sleep, Duration};

use crate::iutils::display_formatted_time;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    start_time: u64,
}

#[allow(non_snake_case)]
pub fn Time(cx: Scope<Props>) -> Element {
    log::debug!("rendering Media time");
    let time = use_state(&cx, || cx.props.start_time);

    cx.spawn({
        to_owned![time];
        async move {
            sleep(Duration::from_millis(1000)).await;
            time += 1;
        }
    });

    let formatted_time = display_formatted_time(**time);
    cx.render(rsx! {
        div {
            class: "media-time",
            "{formatted_time}"
        }
    })
}
