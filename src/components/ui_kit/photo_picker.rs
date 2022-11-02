use dioxus::{prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};
use warp::multipass::{identity::{self, IdentityUpdate}, MultiPass};
use crate::{components::ui_kit::icon_button::{IconButton, self}, utils::config::Privacy};
use rfd::FileDialog;
use crate::{DEFAULT_PATH, Account};
use image_base64::to_base64;

#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>,
    account: Account,
}

#[allow(non_snake_case)]
pub fn PhotoPicker(cx: Scope<Props>) -> Element {
    let account = cx.props.account.clone();
    let identity = account.read().get_own_identity().unwrap();
    let base64_picture = identity.graphics().profile_picture();
    let image_state = use_state(&cx, || base64_picture.clone());
    let show_profile_picture = base64_picture.is_empty();
    
    cx.render(rsx! {
        div {
            class: "photo-picker",
            div {
                class: "display",
                if show_profile_picture {
                    rsx! {
                        Icon {
                            icon: Shape::User,
                            size: 30,
                        },
                    }
                } else {
                    rsx!{
                        img {
                            src: "{image_state}",
                            height: "100",
                            width: "100",
                        }
                    }
                }
            }
            IconButton {
                icon: Shape::Plus,
                on_pressed: move |_| {
                    let path_image = FileDialog::new().add_filter("image", &["jpg", "png", "jpeg"]).set_directory(".").pick_file();
                    match path_image {
                        Some(path) => {
                            let base64_image = to_base64(path.to_str().unwrap());
                            match account.write().update_identity(IdentityUpdate::set_graphics_picture(base64_image)) {
                                Ok(_) => {},
                                Err(e) => {println!("{}", e);}
                            }
                            let identity = account.read().get_own_identity().unwrap();
                            let image = identity.graphics().profile_picture();
                            image_state.set(image);
                        },
                        None => {},
                    }
                    

                }
            },
        }
    })
    

}
