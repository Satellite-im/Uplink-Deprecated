use dioxus::prelude::*;
use dioxus_html::KeyCode;

#[derive(Props)]
pub struct Props<'a> {
    placeholder: String,
    on_submit: EventHandler<'a, String>,
}

// TODO: This is ugly, but we need it for resizing textareas until someone finds a better solution.
const RESIZE_TEXTAREA_SCRIPT: &str = r#"
 (function addAutoResize() {
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
 })()"#;

#[allow(non_snake_case)]
pub fn TextArea<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let text = use_state(&cx, || String::new());
    cx.render(rsx! {
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
            placeholder: "{cx.props.placeholder}"
        }
        script {
            dangerous_inner_html: "{RESIZE_TEXTAREA_SCRIPT}"
        }
    })
}
