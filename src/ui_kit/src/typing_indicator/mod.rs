use dioxus::prelude::*;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    users: Vec<String>,
}

#[allow(non_snake_case)]
pub fn TypingIndicator(cx: Scope<Props>) -> Element {
    let users_list = cx.props.users.clone();
    let name_typing = if users_list.len() <= 3 {
        users_list.join(", ")
    } else {
        users_list.len().to_string() + " users"
    };
    let article = if users_list.is_empty() {
        String::from("Why do i see this indicator? None is")
    } else if users_list.len() == 1 {
        String::from(" is")
    } else {
        String::from(" are")
    };

    cx.render(rsx! {
        div {
            class:"typing-indicator",
            div {
                class: "loader",
            }
            div {
                class: "primary",
                "{name_typing}"
            }
            div {
                class: "secondary",
                "{article} typing..."
            }
        }
    })
}
