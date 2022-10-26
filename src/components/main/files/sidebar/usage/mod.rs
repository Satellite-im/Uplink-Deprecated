use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {
    usage: UsageStats,
}

#[derive(Props, PartialEq)]
pub struct UsageStats {
    pub available: u128,
    pub total: u128,
    pub used: u128,
    pub percent_free: u16,
}

#[allow(non_snake_case)]
pub fn Usage(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        label {
            "Usage"
        },
        div {
            id: "usage",
            div { id: "usage_bar", style: "width:{cx.props.usage.percent_free}%;" },
            div { id: "usage_bar_bg" },
            p {
                id: "usage_text",
                "{cx.props.usage.available}mb free."
            }
        },
    })
}
