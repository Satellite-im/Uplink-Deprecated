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
// Define a state variable to store the current time.
    let time = use_state(&cx, || cx.props.start_time);

// Spawn a new task to increment the time state variable by 1 every second.
    cx.spawn({
        to_owned![time];
        async move {
            // Sleep for 1 second.
            sleep(Duration::from_millis(1000)).await;
            // Increment the time state variable by 1.
            time += 1;
        }
    });

// Format the time to be displayed in the UI.
    let formatted_time = display_formatted_time(**time);
// Render the time in the UI.
    cx.render(rsx! {
        div {
            class: "media-time",
            "{formatted_time}"
        }
    })
}
