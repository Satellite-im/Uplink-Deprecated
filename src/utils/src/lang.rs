use fluent::{FluentBundle, FluentResource};

pub fn translation_or_default(id: &str, resource: &FluentBundle<&'_ FluentResource>) -> String {
    match resource.get_message(id) {
        Some(msg) => {
            let pattern = msg.value().expect("No Translation.");
            let text = resource.format_pattern(&pattern, None, &mut vec![]);
            text.into_owned()
        }
        None => String::from("Unknown"), // TODO: Get default from english version instead,
    }
}
