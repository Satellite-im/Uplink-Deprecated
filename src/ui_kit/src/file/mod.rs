use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use super::folder::State;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    name: String,
    state: State,
    kind: String,
    size: usize,
}

#[allow(non_snake_case)]
pub fn File(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let file_name = format_file_name_to_show(cx);

    let file_size = format_file_size(cx.props.size);

    cx.render(rsx! {
        div {
            class: "folder {class}",
            Icon { icon: Shape::Document },
            p { "{file_name}" },
            label {
                "{file_size}"
            }
        }
    })
}

fn format_file_size(file_size: usize) -> String {
    let base_1024: f64 = 1024.0;
    let size_f64: f64 = file_size as f64;

    let i = (size_f64.log10() / base_1024.log10()).floor();
    let size_formatted = (size_f64 / base_1024.powf(i)).floor();

    let file_size_suffix = ["bytes", "KB", "MB", "GB", "TB"][i as usize];
    return format!("{} {}", size_formatted, file_size_suffix);
}

fn format_file_name_to_show(cx: Scope<Props>) -> String {
    let mut file_name = cx.props.name.clone();

    let file_name_without_extension = std::path::Path::new(&file_name)
        .with_extension("")
        .to_str()
        .unwrap()
        .to_string();

    if file_name_without_extension.len() > 10 {
        file_name = match &file_name.get(0..5) {
            Some(name_sliced) => format!(
                "{}...{}.{}",
                name_sliced,
                file_name_without_extension[file_name_without_extension.len() - 3..].to_string(),
                cx.props.kind
            ),
            None => file_name.clone(),
        };
    }
    file_name
}
