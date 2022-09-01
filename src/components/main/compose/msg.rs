use dioxus::prelude::*;
use sir::global_css;
use warp::raygun::Message;


#[derive(PartialEq, Props)]
pub struct Props {
    message: Message,
    remote: bool,
}

#[allow(non_snake_case)]
pub fn Msg(cx: Scope<Props>) -> Element {
    global_css!("
        .message {
            width: max-content;
            padding: 0.5rem 1rem;
            border-radius: 16px 16px 4px 16px;
            margin: 0.25rem 0;
            font-size: 12px;
            align-self: flex-end;
            background-color: var(--theme-primary);

            &.remote {
                background: var(--theme-foreground-dark);
                align-self: flex-start;
            }
        }
    ");

    let value = cx.props.message.clone().value().join("\n");
    let remote = match cx.props.remote {
        true => "remote",
        false => "local",
    };
    cx.render(rsx! (
        div {
            class: "message {remote}",
            div {
                class: "value",
                "{value}"
            }
        }
    ))
}
