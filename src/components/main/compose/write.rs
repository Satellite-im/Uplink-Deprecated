use dioxus::{events::KeyCode, prelude::*};
use dioxus_heroicons::outline::Shape;
use sir::global_css;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        small_extension_placeholder::SmallExtensionPlaceholder,
    },
    LANGUAGE,
};

#[derive(Props)]
pub struct Props<'a> {
    on_submit: EventHandler<'a, String>,
    on_upload: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Write<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let script = use_state(&cx, String::new);
    // TODO: This is ugly, but we need it for resizing textareas until someone finds a better solution.
    script.set(
        "(function addAutoResize() {
            document.querySelectorAll('.resizeable-textarea').forEach(function (element) {
                element.addEventListener('keyup', function(event) {
                    if (event.keyCode === 13 && !event.shiftKey) {
                        event.target.value = '';
                        event.target.style.height = 'auto';
                    }
                });

                element.style.boxSizing = 'border-box';
                var offset = element.offsetHeight - element.clientHeight;
                element.addEventListener('input', function (event) {
                    event.target.style.height = 'auto';
                    event.target.style.height = event.target.scrollHeight + offset + 'px';
                });
                element.removeAttribute('data-autoresize');
            });
        })()"
            .to_string(),
    );

    global_css!(
        "
        .write {
            flex: 1;
            display: inline-flex;
            flex-direction: row;
            padding: 1rem;
            align-items: end;

            .extension-holder {
                margin-right: 1rem;
            }

            .input {
                flex: 1;
                min-height: 40px;
                color: var(--theme-text);
                border-radius: 4px;
                border: none;
                box-sizing: border-box;
                border: 1px solid transparent;
                border-radius: 20px;
                transition: height .2s, border .2s;
                // TODO: Need help making this prettier, textareas suck
                padding: 0.75rem 1rem 0 1rem;
                margin: 0 1rem;
                word-wrap: break-word;
                word-break: break-all;
                resize: none;
            }
            .input:focus {
                outline: none;
                border: 1px solid var(--theme-primary);
            }
        }
    "
    );

    let text = use_state(&cx, || String::from(""));
    let l = use_atom_ref(&cx, LANGUAGE).read();

    cx.render(rsx! {
        div { class: "write",
            IconButton {
                icon: Shape::Plus,
                on_pressed: move |_| {
                    let _ = &cx.props.on_upload.call(());
                },
            }
            textarea {
                class: "input resizeable-textarea",
                oninput: move |e| {
                    text.set(e.value.clone());
                },
                onkeypress: move |evt| {
                    if evt.key_code == KeyCode::Enter && !evt.shift_key {
                        cx.props.on_submit.call(text.to_string());
                        text.set(String::from(""));
                    }
                },
                placeholder: "{l.chatbar_placeholder}"
            }
            script {
                dangerous_inner_html: "{script}"
            }
            div {
                class: "extension-holder",
                SmallExtensionPlaceholder {}
            }
            IconButton {
                icon: Shape::ArrowRight,
                state: icon_button::State::Secondary,
                on_pressed: move |_| {
                    let _ = &cx.props.on_submit.call(text.to_string());
                },
            }
        }
    })
}
