use dioxus::prelude::*;
use regex::RegexSet;
use sir::css;
use crate::{Account, LANGUAGE};
use dioxus::events::FormEvent;
use ui_kit::{
    button::{self, Button},
    input::Input,
    photo_picker::PhotoPicker,
};
use warp::multipass::identity::{IdentityUpdate};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[inline_props]
#[allow(non_snake_case)]
pub fn Profile(cx: Scope<Props>, account:Account) -> Element {
    log::debug!("rendering settings/pages/Profile");
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2= l.clone();

    let account2 = account.clone();
    let identity = account2.read().get_own_identity().unwrap();

    let user_name = identity.username();
    let user_name2 = user_name.clone();
    let user_name_state = use_state(&cx, || user_name.clone());
    let edit_user_name_state = use_state(&cx, || false);
    let user_name_error = use_state(&cx, String::new);
    let user_name_error_class = if user_name_error.is_empty() {
        css!("opacity: 0")
    } else {
            "error_text"
        };


    let status_msg = match identity.status_message(){
        Some(msg)=> msg,
        None => String::new()
    };
    let status_msg2 = status_msg.clone();
    let status_msg_state = use_state(&cx, || status_msg.clone());
    let edit_status_msg_state = use_state(&cx, || false);
    let status_msg_error = use_state(&cx, String::new);
    let status_msg_error_class = if status_msg_error.is_empty() {
        css!("opacity: 0")
    } else {
            "error_text"
        };


    fn update_user_name(user_name_state:&UseState<String>, user_name:&String,user_name_error:&UseState<String>,account:&Account,edit_user_name_state:&UseState<bool>){
        let user_name_text = user_name_state.trim();
        if user_name_text != *user_name{
                if user_name_text.is_empty(){
                        user_name_error.set("Username is required".into())
                }else if user_name_text.len() < 4 ||user_name_text.len() > 32  {
                        user_name_error.set("Username needs to be between 4 and 32 characters long".into())
                }else{
                let user_name_regex_set = RegexSet::new(&[
                            r"@",
                            r"\p{Emoji_Presentation}",
                 ]).unwrap();
                let matches = user_name_regex_set.matches(user_name_text);
                if matches.matched(0){
                    user_name_error.set("@ is not allowed in username".into());
                } else if matches.matched(1){
                    user_name_error.set("emoji is not allowed in username".into());    
                } else {
                    if let Err(e) =  account.write().update_identity(IdentityUpdate::set_username(
                        user_name_text.to_string()))
                        {
                        println!("Failed in updating status message:{}", e);
                        }
                        edit_user_name_state.set(false);
                }
            }
        }

    }



    fn update_status_msg(status_msg_state:&UseState<String>,status_msg: &String,status_msg_error: &UseState<String>, account:&Account, edit_status_msg_state:&UseState<bool> ){
         let status_msg_text = status_msg_state.trim();
                                    if status_msg_text != *status_msg{
                                        if status_msg_text.len() > 128{
                                            status_msg_error.set("status message needs to be less than 128 characters".into());

                                        }else{
                                            if let Err(e) =  account.write().update_identity(IdentityUpdate::set_status_message(
                                                Some(status_msg_state.to_string(),)))
                                                {
                                                println!("Failed in updating status message:{}", e);
                                                }
                                            edit_status_msg_state.set(false);
                                        }   
                                    }  

    }

    cx.render(rsx! {
        div {
            id: "page_profile",
            class: "padded",
            div {
                class: "profile-header",
                div {
                    class: "profile-picture",
                    PhotoPicker {
                        account: cx.props.account.clone(),
                    },
                }
            },
            div {
                div{
                    label {
                        "User Name"
                    },
                    if **edit_user_name_state {rsx! (
                        div {
                            class: "change-profile",
                            div{
                                class: "input-profile",
                                Input {
                                    placeholder: "type user name".to_string(),
                                    value: user_name_state.to_string(),
                                    on_change: move |e: FormEvent| {
                                        user_name_error.set("".into());
                                        user_name_state.set(e.value.clone());
                                    },
                                    on_enter:move|_|{
                                        update_user_name(user_name_state,&user_name,user_name_error,account,edit_user_name_state);
                            },
                        },
                    },
                    Button {
                                text: l.save.to_string(),
                                on_pressed: move |_|{
                                    update_user_name(user_name_state,&user_name2,user_name_error,account,edit_user_name_state);
                                }
                            }
                        },
                    )} else {rsx! (
                        div{
                            class: "change-profile",
                            span {
                                "{user_name}"
                        },
                           Button {
                                    text: l.edit.to_string(),
                                    state: button::State::Secondary,
                                    on_pressed: move |_| {
                                        edit_user_name_state.set(true);
                                    },
                            },
                        },)
                    },
                    p {
                        class: "{user_name_error_class}",
                        "{user_name_error}"
                    },
                }
                div{
                    label {
                        "Status Message"
                    },
                if **edit_status_msg_state {rsx! (
                    div {
                        class: "change-profile",
                        div{
                            class: "input-profile",
                            Input {
                                placeholder: "type".to_string(),
                                value: status_msg_state.to_string(),
                                on_change: move |e: FormEvent| status_msg_state.set(e.value.clone()),
                                on_enter:move|_|{
                                    update_status_msg(status_msg_state,&status_msg,user_name_error,account,edit_status_msg_state);
                                    }   
                                },
                            },
                            Button {
                                text: l2.save.to_string(),
                              on_pressed: move |_|{
                                  update_status_msg(status_msg_state,&status_msg2,user_name_error,account,edit_status_msg_state);
                              }
                          },
                        },              
                        
                )} else {rsx! (
                    div{
                        class: "change-profile",
                        span {
                            "{status_msg}",
                        },
                       Button {
                        text: l2.edit.to_string(),
                        state: button::State::Secondary,
                                on_pressed: move |_| {
                                    edit_status_msg_state.set(true);
                                },
                            },
                        },)
                }
                p {
                    class: "{status_msg_error_class}",
                    "{status_msg_error}"
                },}
            },
        }
    })
}
