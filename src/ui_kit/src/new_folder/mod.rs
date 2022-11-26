use dioxus::prelude::*;
use dioxus_elements::input_data::keyboard_types::Code;
use dioxus_heroicons::{solid::Shape, Icon};

use super::folder::State;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    state: State,
}

#[allow(non_snake_case)]
pub fn NewFolder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let folder_name = use_state(&cx, || String::from("New Folder"));

    cx.render(rsx! {
        div {
            class: "folder {class}",
            Icon { icon: Shape::Folder },
            input {
                class: "new_folder_input",
                autofocus: "true",
                placeholder: "New Folder",
                oninput: move |evt| {
                    folder_name.set(evt.value.to_string());
                },
                onkeyup: |evt| {
                    if evt.code().eq(&Code::Enter) {
                        println!("Create new folder: {}", folder_name.clone());
                    }
                }
            }
        }
    })
}
