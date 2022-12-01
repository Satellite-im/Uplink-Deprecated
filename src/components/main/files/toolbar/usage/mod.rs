use dioxus::prelude::*;
use humansize::format_size;
use humansize::DECIMAL;

#[derive(Props, PartialEq, Eq)]
pub struct UsageContentProps {
    space: String,
    available: u128,
}

#[allow(non_snake_case)]
pub fn UsageContent(cx: Scope<UsageContentProps>) -> Element {
    cx.render(rsx! {
        Fragment {
            div {
                class: "usage_bar_heading ellipsis",
                "{cx.props.available} Free",
            }
            div {
                class: "usage_bar_subheading ellipsis",
                "{cx.props.space}"
            }
        }
    })
}

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
    let free_space = fs2::free_space("/").unwrap_or(1);
    let total_space = fs2::total_space("/").unwrap_or(1);
    let perc = (((total_space / free_space) as f64) * 0.1) * 100.0;
    let space = format!(
        "{} / {}",
        format_size(free_space, DECIMAL),
        format_size(total_space, DECIMAL)
    );

    cx.render(rsx! {
        div {
            id: "usage",
            div {
                id: "usage_bar_bg",
                UsageContent {
                    space: space.clone(),
                    available: cx.props.usage.available
                }
            },
            div {
                id: "usage_bar",
                style: "-webkit-clip-path: inset(0 0 0 {perc}%);",
                UsageContent {
                    space: space.clone(),
                    available: cx.props.usage.available
                }
            },
        },
    })
}
