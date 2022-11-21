use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

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
pub fn File<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    let file_name = format_file_name_to_show(cx);

    let file_size = format_file_size(cx.props.size);

    cx.render(rsx! {
        div {
            class: "folder {class}",

                Icon { icon: Shape::Document},
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
    let size_formatted = size_f64 / base_1024.powf(i);

    let file_size_suffix = ["bytes", "KB", "MB", "GB", "TB"][i as usize];
    let mut size_formatted_string = format!(
        "{size:.*} {size_suffix}",
        1,
        size = size_formatted,
        size_suffix = file_size_suffix
    );
    if size_formatted_string.contains(".0") {
        size_formatted_string = size_formatted_string.replace(".0", "");
    }
    size_formatted_string
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
                &file_name_without_extension[file_name_without_extension.len() - 3..].to_string(),
                cx.props.kind
            ),
            None => file_name.clone(),
        };
    }
    file_name
}
