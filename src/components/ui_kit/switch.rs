use dioxus::{core::UiEvent, events::FormData, prelude::*};

// Remember: owned props must implement PartialEq!
#[derive(Props)]
pub struct Props<'a> {
    active: bool,
    // TODO: we should insted just return a bool with the switche's binary state
    on_change: EventHandler<'a, UiEvent<FormData>>,
}

pub fn css() -> String {
    "
    .switch {
        position: relative;
        display: inline-block;
        width: 36px;
        height: 16px;
        margin-top: 2px;
    }
    
    .switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }
    
    .slider {
        user-select: none;
        position: absolute;
        cursor: pointer;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: var(--theme-foreground);
        -webkit-transition: .2s;
        transition: .2s;
        border-radius: 10px;
    }
    
    .slider:before {
        position: absolute;
        content: '';
        height: 20px;
        width: 20px;
        left: 0;
        bottom: -2px;
        border-radius: 10px;
        background-color: var(--theme-text-bright);
        -webkit-transition: .2s;
        transition: .2s;
    }
    
    input:checked + .slider {
        background-color: var(--theme-primary);
    }
    
    input:focus + .slider {
        box-shadow: 0 0 1px var(--theme-primary);
    }
    
    input:checked + .slider:before {
        transform: translateX(16px);
    }
    "
    .to_string()
}

#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx! {
        label {
            class: "switch",
            input {
                "type": "checkbox",
                checked: "{cx.props.active}",
                oninput: move |evt| cx.props.on_change.call(evt)
            },
            span {
                class: "slider",
            }
        }
    })
}
