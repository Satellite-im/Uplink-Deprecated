use dioxus::{prelude::*, events::{FormEvent}};

#[derive(PartialEq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    placeholder: String,
    #[props(optional)]
    text: Option<String>,
    oninput: EventHandler<'a, FormEvent>
}

pub fn css() -> String {"
    .input {
        min-width: 100px;
        width: 100%;
        height: 40px;
        padding-left: 10px;
        padding-right: 10px;
        color: var(--theme-text);
        border-radius: 4px;
        border: none;
        background: var(--theme-secondary);
        box-sizing: border-box;
        border: 1px solid transparent;
        transition: .2s;
        margin: 1px;
    }
    .input:focus {
        outline: none;
        border: 1px solid var(--theme-primary);
        margin: 1px;
    }
    ".to_string()}

#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx!{
            input {
                class: "input",
                placeholder: "{cx.props.placeholder}",
                oninput: move |evt| cx.props.oninput.call(evt),
            }
        }
    )
}