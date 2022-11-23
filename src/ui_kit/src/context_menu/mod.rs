use dioxus::{
    desktop::{tao::window, use_window},
    prelude::*,
};
use dioxus_heroicons::{outline::Shape, Icon};

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
    #[props(optional)]
    devmode: Option<bool>,
}

#[allow(non_snake_case)]
pub fn ContextMenu<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    // Handles the hiding and showing of the context menu
    let script = include_str!("./context.js").replace("ID", &cx.props.parent);

    let id = format!("{}-context-menu", &cx.props.parent);
    let window = use_window(&cx);

    cx.render(rsx! {
        div {
            id: "{id}",
            class: "context-menu hidden",
            &cx.props.items,
            cx.props.devmode.is_some().then(|| rsx!(
                hr {},
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
