use dioxus::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props {
    usage: UsageStats,
}

#[derive(Props, PartialEq, Eq)]
pub struct UsageStats {
    pub available: u128,
    pub total: u128,
    pub used: u128,
    pub percent_free: u16,
}

#[allow(non_snake_case)]
pub fn Usage(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            div {
                id: "usage",
                div {
                    id: "usage_bar",
                    style: "width:{cx.props.usage.percent_free}%;",
                    (cx.props.usage.percent_free > 60).then(||  rsx!{
                        span {
                            class: "usage-available-text",
                            "{cx.props.usage.available} MB Free",
                        }
                    })
                },
                div {
                    id: "usage_bar_bg",
                    (cx.props.usage.percent_free <= 59).then(||  rsx!{
                        span {
                            class: "usage-available-text",
                            "{cx.props.usage.available} MB Free",
                        }
                    })
                },
            },
        }
    })
}
