use dioxus::{prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};
use warp::multipass::{identity::{IdentityUpdate}};
use crate::{components::ui_kit::icon_button::{IconButton}};
use rfd::FileDialog;
use crate::{Account};

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
                            class: "profile_photo",
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
                            
                            let file = std::fs::read(&path).unwrap();

                            let prefix = String::from("data:image/png;base64,");

                            let base64_image = base64::encode(&file);

                            match account.write().update_identity(IdentityUpdate::set_graphics_picture(prefix + base64_image.as_str())) {
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
