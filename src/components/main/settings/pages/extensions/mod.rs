use audio_factory::AudioFactory;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use emoji_selector::EmojiSelector;
use incognito_typing::ExtIncognitoTyping;
use ui_kit::button::Button;
use utils::extensions::{BasicExtension, ExtensionInfo};

pub mod extension;

use crate::components::{
    main::settings::pages::extensions::extension::ExtensionOptions, reusable::toolbar::Toolbar,
};

#[allow(non_snake_case)]
pub fn Extensions(cx: Scope) -> Element {
    log::debug!("rendering settings/pages/Extensions");

    let extensions: Vec<ExtensionInfo> = vec![
        AudioFactory::info(),
        EmojiSelector::info(),
        ExtIncognitoTyping::info(),
    ];

    let extensions_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".warp/extensions")
        .into_os_string()
        .into_string()
        .unwrap_or_default();

    cx.render(rsx! {
        div {
            id: "page_extensions",
            Toolbar {
                controls: cx.render(rsx! {
                    Button {
                        text: String::from("Extensions Folder"),
                        icon: Shape::FolderOpen,
                        on_pressed: move |_| {
                            let _ = opener::open(&extensions_path);
                        }
                    }
                })
            },
            div {
                id: "extensions",
                extensions.iter().map(|e| rsx!{ ExtensionOptions { extension: e.clone() } })
            }
        }
    })
}
