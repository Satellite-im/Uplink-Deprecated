use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    options: Vec<String>,
    on_change: EventHandler<'a, String>,
}

#[allow(non_snake_case)]
pub fn Select<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let iter = IntoIterator::into_iter(cx.props.options.clone());
    cx.render(rsx!(div { class: "select", select {
        onchange: move |e| cx.props.on_change.call(e.value.clone()),
        iter.map(|val| rsx!(
            option { label: "{val}", value: "{val}" }
        ))
    }}))
}
