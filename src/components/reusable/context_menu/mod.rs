use dioxus::{desktop::use_window, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};

use crate::iutils::config::Config;

#[derive(Props)]
pub struct ItemProps<'a> {
    onpressed: EventHandler<'a>,
    text: String,
    #[props(optional)]
    icon: Option<Shape>,
    #[props(optional)]
    danger: Option<bool>,
}

#[allow(non_snake_case)]
pub fn ContextItem<'a>(cx: Scope<'a, ItemProps<'a>>) -> Element<'a> {
    let class = if cx.props.danger.is_some() {
        "context-item danger"
    } else {
        "context-item"
    };
    cx.render(rsx! {
        div {
            class: "{class}",
            onclick: move |_| cx.props.onpressed.call(()),
            (cx.props.icon.is_some()).then(|| {
                let icon = match cx.props.icon {
                    Some(shape) => shape,
                    None => Shape::Cog,
                };
                rsx! {
                    Icon { icon: icon }
                }
            }),
            p {"{cx.props.text}"}
        }
    })
}

#[derive(Props)]
pub struct Props<'a> {
    parent: String,
    items: Element<'a>,
}

#[allow(non_snake_case)]
pub fn ContextMenu<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let config = Config::load_config_or_default();

    // Handles the hiding and showing of the context menu
    let script = include_str!("./context.js").replace("ID", &cx.props.parent);

    // Used for developer options
    let window = use_window(&cx);
    let cache_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".warp")
        .into_os_string()
        .into_string()
        .unwrap_or_default();

    let id = format!("{}-context-menu", &cx.props.parent);

    cx.render(rsx! {
        div {
            id: "{id}",
            class: "context-menu hidden",
            &cx.props.items,
            config.developer.developer_mode.then(|| rsx!(
                hr {},
                ContextItem {
                    icon: Shape::FolderOpen,
                    text: String::from("Open Cache"),
                    onpressed: move |_| {
                        let _ = opener::open(&cache_path);
                    },
                }
                ContextItem {
                    icon: Shape::Terminal,
                    text: String::from("Open Console"),
                    onpressed: move |_| window.devtool(),
                }
            ))
        },
        script { "{script}" }
    })
}
