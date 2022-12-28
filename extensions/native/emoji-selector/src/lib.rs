use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use emojis::{Group, UnicodeVersion};
use sir::css;
use ui_kit::{
    button::{self, Button},
    outside::OutsideClick,
};
use utils::extensions::{BasicExtension, ExtensionInfo, ExtensionType};

static MAX_UNICODE_VER: UnicodeVersion = UnicodeVersion::new(11, 0);

fn get_group_name(group: Group) -> String {
    match group {
        Group::SmileysAndEmotion => "Smiles & Emotions".to_string(),
        Group::PeopleAndBody => "People & Body".to_string(),
        Group::AnimalsAndNature => "Animals & Nature".to_string(),
        Group::FoodAndDrink => "Food & Drink".to_string(),
        Group::TravelAndPlaces => "Travel & Places".to_string(),
        Group::Activities => "Activities".to_string(),
        Group::Objects => "Objects".to_string(),
        Group::Symbols => "Symbols".to_string(),
        Group::Flags => "Flags".to_string(),
    }
}

pub struct EmojiSelector;

impl BasicExtension for EmojiSelector {
    fn info() -> ExtensionInfo {
        ExtensionInfo {
            name: String::from("Emoji Picker"),
            author: String::from("matt@satellite.im"),
            description: String::from(
                "Select emoji's from an organized list of all supported emojis. Also provides tooling to transcribe text names into emoji icons.",
            ),
            location: ExtensionType::ChatbarIcon,
        }
    }

    fn render(cx: Scope) -> Element {
        let styles = css!(
            "
            border: 1px solid var(--theme-borders);
            background: var(--theme-background);
            padding: 0.25rem;
            position: absolute;
            z-index: 5;
            border-radius: 8px;
            bottom: 80px;
            width: 600px;
            max-height: 60vh;
            right: 20px;
            max-width: calc(100% - 20px); // -20px is based on the 'right' attribute above
            overflow-y: scroll;
            &::-webkit-scrollbar-thumb {
                background: var(--theme-primary) !important;
                opacity: 1;
            }
            .category {
                display: inline-flex;
                flex-direction: column;
                margin-bottom: 0.5rem;
                padding: 0.25rem;
                width: 100%;

                .items {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(30px, 1fr));
                    .item {
                        cursor: pointer;
                    }
                }
            }
        "
        );

        let eval = use_eval(&cx);
        let insert = move |val: &str| {
            eval(format!(
                "
                document.querySelector('#TODO-textarea-input .dynamic-input').focus();
                document.execCommand('insertText', false, '{}');
            ",
                val
            ));
        };

        let groups = Group::iter();
        cx.render(rsx! {
                div {
                    class: "ext-emoji-selector",
                    rsx! {
                        OutsideClick {
                        div {
                            onblur: |_| println!("blur"),
                            class: "{styles}",
                            groups.map(|group| {
                                let name = get_group_name(group);
                                rsx!(
                                    div {
                                        class: "category",
                                        div {
                                            class: "name",
                                            label { "{name}" }
                                        },
                                        div {
                                            class: "items",
                                            group.emojis()
                                                .filter(|v| v.unicode_version() <= MAX_UNICODE_VER)
                                                .map(|v| {
                                                    let name = v.name();
                                                    let emoji = v.as_str();
                                                    rsx!(button {
                                                        onclick: move |_| insert(emoji),
                                                        class: "item",
                                                        title: "{name}",
                                                        "{v}"
                                                    })
                                                })
                                        },
                                    }
                                )
                            })
                        }
                        Button {
                            icon: Shape::FaceSmile,
                            on_pressed: move |_| {}
                        }
                    }
                } 
            }
        })
    }
}

#[no_mangle]
pub extern "C" fn ret_rend() -> Box<fn(Scope) -> Element> {
    Box::new(EmojiSelector::render)
}

#[no_mangle]
pub extern "C" fn ret_info() -> Box<ExtensionInfo> {
    Box::new(EmojiSelector::info())
}
