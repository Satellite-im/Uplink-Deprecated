use dioxus::prelude::*;

use ui_kit::button::Button;

#[derive(PartialEq, Eq)]
pub enum Route {
    General,
    Privacy,
    AudioVideo,
    Extensions,
    Developer,
    Profile,
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    text: String,
    active: bool,
    disabled: bool,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn NavButton<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let class = if cx.props.active {
        "active"
    } else {
        "inactive"
    };

    cx.render(rsx!(
        div {
            class: "nav-button {class}",
            Button {
                on_pressed: move |_| cx.props.on_pressed.call(()),
                disabled: cx.props.disabled,
                text: cx.props.text.clone()
            }
        }
    ))
}

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, Route>,
    initial_value: Route,
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::debug!("rendering settings/sidebar/Nav ");
    let initial_value = match cx.props.initial_value {
        Route::Profile => Route::Profile,
        Route::Developer => Route::Developer,
        _ => Route::General,
    };
    let active_item = use_state(&cx, || initial_value);

    cx.render(rsx! {
        div {
            class: "column_navigation",
            NavButton {
                text: String::from("General"),
                active: Route::General.eq(active_item),
                disabled: false,
                on_pressed: |_| {
                    active_item.set(Route::General);
                    cx.props.on_pressed.call(Route::General);
                }
            },
            NavButton {
                text: String::from("Profile"),
                active: Route::Profile.eq(active_item),
                disabled: false,
                on_pressed: |_| {
                    active_item.set(Route::Profile);
                    cx.props.on_pressed.call(Route::Profile);
                }
            },
            NavButton {
                text: String::from("Privacy"),
                active: Route::Privacy.eq(active_item),
                disabled: true,
                on_pressed: |_| {
                    active_item.set(Route::Privacy);
                    cx.props.on_pressed.call(Route::Privacy);
                }
            },
            NavButton {
                text: String::from("AudioVideo"),
                active: Route::AudioVideo.eq(active_item),
                disabled: false,
                on_pressed: |_| {
                    active_item.set(Route::AudioVideo);
                    cx.props.on_pressed.call(Route::AudioVideo);
                }
            },
            NavButton {
                text: String::from("Extensions"),
                active: Route::Extensions.eq(active_item),
                disabled: false,
                on_pressed: |_| {
                    active_item.set(Route::Extensions);
                    cx.props.on_pressed.call(Route::Extensions);
                }
            },
            NavButton {
                text: String::from("Developer"),
                active: Route::Developer.eq(active_item),
                disabled: false,
                on_pressed: |_| {
                    active_item.set(Route::Developer);
                    cx.props.on_pressed.call(Route::Developer);
                }
            }
        }
    })
}
