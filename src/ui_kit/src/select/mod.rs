use dioxus::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Props)]
pub struct Props<'a> {
    options: Vec<SelectOption>,
    value: String,
    on_change: EventHandler<'a, String>,
}

#[allow(non_snake_case)]
pub fn Select<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let options = IntoIterator::into_iter(cx.props.options.clone());
    cx.render(rsx!(
        div {
            class: "select",
            select {
                onchange: move |e| cx.props.on_change.call(e.value.clone()),
                options.map(|option| {
                    let selected = option.value == cx.props.value;
                    rsx!(
                        option {
                            label: "{option.label}",
                            value: "{option.value}",
                            selected: "{selected}",
                        }
                    )
                })
            }
        }
    ))
}
