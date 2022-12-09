use chrono::prelude::*;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {
    date: DateTime<Utc>,
    num_unread: u32,
}

#[allow(non_snake_case)]
pub fn Divider<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let num_unread = **(use_state(&cx, || cx.props.num_unread));
    let date = cx.props.date.with_timezone(&Local);
    let label_date = if date.date() == Local::today() {
        date.format("%R").to_string()
    } else {
        date.format("%R %x").to_string()
    };

    let label = format!(
        "{} new message{} since {}",
        num_unread,
        if num_unread == 1 { "" } else { "s" },
        label_date,
    );

    cx.render(rsx! {
        div {
            class: "msg-divider",
            div {
                class: "msg-divider-label",
                "{label}"
            }
            div {
                class: "msg-divider-line",
            }
        }
    })
}
