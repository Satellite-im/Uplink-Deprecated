use dioxus::prelude::*;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    enabled: bool,
    #[props(optional)]
    debug: Option<bool>,
}

#[allow(non_snake_case)]
pub fn ExtAudioFactory(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            "todo"
        }
    })
}

#[derive(PartialEq, Eq, Props)]
pub struct ControlProps {
    on_pressed: bool,
}

pub fn ExtAudioFactoryControl(cx: Scope<ControlProps>) -> Element {
    cx.render(rsx! {
        div {
            "todo"
        }
    })
}
