use dioxus::{desktop::use_window, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(Props)]
pub struct ItemProps<'a> {
    onpressed: EventHandler<'a>,
    text: String,
    #[props(optional)]
    icon: Option<Shape>,
}

#[allow(non_snake_case)]
pub fn ContextItem<'a>(cx: Scope<'a, ItemProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "context-item",
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

    cx.render(rsx! {
        div {
            id: "context-menu",
            class: "hidden",
            &cx.props.items,
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
        },
        script { "{script}" }
    })
}
