use dioxus::prelude::*;
use humansize::format_size;
use humansize::DECIMAL;

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
    let free_space = match fs2::free_space("/") {
        Ok(space) => space,
        Err(_) => 1,
    };
    let total_space = match fs2::total_space("/") {
        Ok(space) => space,
        Err(_) => 1,
    };
    let perc = (((total_space / free_space) as f64) * 0.1) * 100.0;
    let space = format!(
        "{}/{} Free",
        format_size(free_space, DECIMAL),
        format_size(total_space, DECIMAL)
    );
    let space_clone = space.clone();

    cx.render(rsx! {
        div {
            id: "usage",
            div {
                id: "usage_bar",
                style: "width:{perc}%;",
                (perc > 60.0).then(||  rsx!{
                    span {
                        class: "usage-available-text",
                        "{cx.props.usage.available} Free",
                        br {},
                        span {
                            "Disk Space: {space}"
                        }
                    }
                })
            },
            div {
                id: "usage_bar_bg",
                (perc <= 59.0).then(||  rsx!{
                    span {
                        class: "usage-available-text",
                        "{cx.props.usage.available} Free",
                        br {},
                        span {
                            "Disk Space: {space_clone}"
                        }
                    }
                })
            },
        },
    })
}
