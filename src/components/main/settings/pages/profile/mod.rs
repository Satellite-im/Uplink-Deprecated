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

    let username = identity.username();
    let username2 = username.clone();
    let username_state = use_state(&cx, || username.clone());
    let edit_username_state = use_state(&cx, || false);
    let username_error = use_state(&cx, String::new);
    let username_error_class = if username_error.is_empty() {
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


    fn update_username(username_state:&UseState<String>, username:&String,username_error:&UseState<String>,account:&Account,edit_username_state:&UseState<bool>){
        let username_text = username_state.trim();
        if username_text != *username{
                if username_text.is_empty(){
                        username_error.set("Username is required".into())
                }else if username_text.len() < 4 ||username_text.len() > 32  {
                        username_error.set("Username needs to be between 4 and 32 characters long".into())
                }else{
                let username_regex_set = RegexSet::new(&[
                            r"@",
                            r"\p{Emoji_Presentation}",
                 ]).unwrap();
                let matches = username_regex_set.matches(username_text);
                if matches.matched(0){
                    username_error.set("@ is not allowed in username".into());
                } else if matches.matched(1){
                    username_error.set("emoji is not allowed in username".into());    
                } else {
                    if let Err(e) =  account.write().update_identity(IdentityUpdate::set_username(
                        username_text.to_string()))
                        {
                        println!("Failed in updating status message:{}", e);
                        }
                        edit_username_state.set(false);
                }
            }
        } else {
            edit_username_state.set(false);
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
                                    }  else {
                                        edit_status_msg_state.set(false);
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
                    if **edit_username_state {rsx! (
                        div {
                            class: "change-profile",
                            div{
                                class: "input-profile",
                                Input {
                                    placeholder: "Digit your username...".to_string(),
                                    value: username_state.to_string(),
                                    on_change: move |e: FormEvent| {
                                        username_error.set("".into());
                                        username_state.set(e.value.clone());
                                    },
                                    on_enter:move|_|{
                                        update_username(username_state,&username,username_error,account,edit_username_state);
                            },
                        },
                    },
                    Button {
                                text: l.save.to_string(),
                                on_pressed: move |_|{
                                    update_username(username_state,&username2,username_error,account,edit_username_state);
                                }
                            }
                        },
                    )} else {rsx! (
                        div{
                            class: "change-profile",
                            span {
                                "{username}"
                        },
                           Button {
                                    text: l.edit.to_string(),
                                    state: button::State::Secondary,
                                    on_pressed: move |_| {
                                        edit_username_state.set(true);
                                    },
                            },
                        },)
                    },
                    p {
                        class: "{username_error_class}",
                        "{username_error}"
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
                                placeholder: "Digit your status message...".to_string(),
                                value: status_msg_state.to_string(),
                                on_change: move |e: FormEvent| status_msg_state.set(e.value.clone()),
                                on_enter:move|_|{
                                    update_status_msg(status_msg_state,&status_msg,status_msg_error,account,edit_status_msg_state);
                                    }   
                                },
                            },
                            Button {
                                text: l2.save.to_string(),
                              on_pressed: move |_|{
                                  update_status_msg(status_msg_state,&status_msg2,status_msg_error,account,edit_status_msg_state);
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
