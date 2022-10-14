use dioxus::{prelude::*};
use dioxus_html::KeyCode;
use dioxus_heroicons::outline::Shape;
use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        small_extension_placeholder::SmallExtensionPlaceholder,
    },
    LANGUAGE, utils::config::Config,
};

#[derive(Props)]
pub struct Props<'a> {
    on_submit: EventHandler<'a, String>,
    on_upload: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Write<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let config = Config::load_config_or_default();

    let script = use_state(&cx, String::new);
    // TODO: This is ugly, but we need it for resizing textareas until someone finds a better solution.
    script.set(
        "(function addAutoResize() {
            document.querySelectorAll('.resizeable-textarea').forEach(function (element) {
                let send_button = document.getElementById('send');
                send_button.addEventListener('click', function(event) {
                    element.value = '';
                });

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

    let text = use_state(&cx,  String::new);
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
                    if evt.key_code == KeyCode::Enter  {
                        cx.props.on_submit.call(text.to_string());
                        text.set(String::from(""));
                    }
                },
                placeholder: "{l.chatbar_placeholder}"
            }
            script {
                dangerous_inner_html: "{script}"
            }
            config.developer.developer_mode.then(|| rsx! {
                div {
                    class: "extension-holder",
                    SmallExtensionPlaceholder {}
                }
            })
            div {
                id: "send",
                IconButton {
                    icon: Shape::ArrowRight,
                    state: icon_button::State::Secondary,
                    on_pressed: move |_| {
                        let _ = &cx.props.on_submit.call(text.to_string());
                        text.set(String::from(""));
                    },
                }
            }
        }
    })
}
