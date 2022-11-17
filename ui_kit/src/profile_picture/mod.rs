use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub enum Size {
    Large,
    Normal,
    Small,
}

// Remember: owned props must implement PartialEq!
#[derive(Props, PartialEq, Eq)]
pub struct Props {
    #[props(!optional)]
    src: Option<String>,
    size: Size, // todo: why is this unused?
}

#[allow(non_snake_case)]
pub fn PFP(cx: Scope<Props>) -> Element {
    let style = match &cx.props.src {
        Some(img) => format!("background-image: url({});", img),
        None => "".into(),
    };

    cx.render(rsx! {
        div {
            class: "pfp",
            style: "{style}"
        }
    })
}
