use arboard::Clipboard;
use dioxus::{
    core::UiEvent,
    events::{FormEvent, MouseData},
    prelude::*,
};
use dioxus_heroicons::outline::Shape;
use dioxus_toast::{Position, ToastInfo};

use crate::{Account, LANGUAGE, TOAST_MANAGER};

use ui_kit::{
    button::Button,
    input::{Input, SelectOption},
};

use warp::{crypto::DID, multipass::identity::Identifier};

#[inline_props]
#[allow(non_snake_case)]
pub fn FindFriends(
    cx: Scope,
    account: Account,
    add_error: UseState<String>,
    is_compact: bool,
) -> Element {
    let toast = use_atom_ref(&cx, TOAST_MANAGER);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let remote_friend = use_state(&cx, String::new);

    let l2 = l.clone();
    let l3 = l.clone();
    let codeCopied = { l.code_copied.to_string() };
    let account2 = account.clone();

    let copy_friend_id = move || {
        let mut clipboard = Clipboard::new().unwrap();
        if let Ok(ident) = account2.get_own_identity() {
            let single_toast = ToastInfo {
                position: Position::TopRight,
                ..ToastInfo::simple(&codeCopied)
            };
            let _id = toast.write().popup(single_toast); // copy to the clipboard without prefix 'did:key:'
            clipboard
                .set_text(&ident.did_key().to_string()[8..])
                .unwrap();
        }
    };
    let copy_friend_id2 = copy_friend_id.clone();

    let search_results = use_state(&cx, Vec::<SelectOption>::new);

    let on_change = move |ev: FormEvent| {
        let value = ev.data.value.clone();

        remote_friend.set(value.clone());

        if value.len() < 3 {
            if !search_results.is_empty() {
                search_results.set(Vec::new());
            }

            return;
        }

        match account.get_identity(Identifier::user_name(value.clone().as_str())) {
            Ok(results) => {
                let opts = results
                    .iter()
                    .map(|result| SelectOption {
                        value: result.did_key().to_string().replace("did:key:", ""),
                        label: format!(
                            "{}#{}",
                            result.username().to_string(),
                            result.short_id().to_string()
                        ),
                    })
                    .collect();

                search_results.set(opts);
            }
            Err(_) => {}
        };
    };

    cx.render(rsx!(
        div {
            id: "find-friends",
            label {
                "{l.add_someone}",
            },
            div {
                class: "add",  
                Input {
                    placeholder: l.add_placeholder.clone(),
                    on_change: on_change,
                    on_enter: move |_| {}
                    options: search_results.get().clone(),
                    on_item_selected: move |item:String| {
                        remote_friend.set(item.clone());
                        search_results.set(Vec::new());
                    },
                    value: remote_friend.get().clone(),
                }
                Button {
                    icon: Shape::Plus,
                    on_pressed: move |e: UiEvent<MouseData>| {
                        e.cancel_bubble();

                        let did = DID::try_from(format!("did:key:{}", remote_friend.clone()));
                        match did {
                            Ok(d) => {
                                match account.clone()
                                    .send_request(&d)
                                {
                                    Ok(_) => {
                                        let single_toast = ToastInfo {
                                            position: Position::TopRight,
                                            ..ToastInfo::simple(&l2.request_sent)
                                        };
                                        let _id = toast.write().popup(single_toast);
                                        add_error.set("".into());
                                    }
                                    Err(e) => {
                                        add_error.set(match e {
                                            warp::error::Error::CannotSendFriendRequest => l2.couldnt_send.to_string(),
                                            warp::error::Error::FriendRequestExist => l2.already_sent.to_string(),
                                            warp::error::Error::CannotSendSelfFriendRequest => l2.add_self.to_string(),
                                            warp::error::Error::FriendExist => l2.friend_exist.to_string(),
                                            _ => l2.something_went_wrong.to_string()
                                        })
                                    },
                                };
                            },
                            Err(_) => add_error.set(l2.invalid_code.to_string()),
                        }
                        remote_friend.set("".into());
                    },
                },
                is_compact.then(|| rsx!{
                    span {
                        title: "{l2.copy_friend_code}",
                        Button {
                            icon: Shape::ClipboardDocument,
                            on_pressed: move |e: UiEvent<MouseData>| {
                                e.cancel_bubble();
                                copy_friend_id();
                            }
                        }
                    }
                }),
            },
            div {
                class: "error_text",
                "{add_error}"
            },
            (!is_compact).then(|| rsx!{
                div {
                    class: "copy-friend-code",
                    label {
                        "{l3.copy_friend_code}",
                    },
                    div {
                        class: "code",
                        title: "{l3.copy_friend_code}",
                        Button {
                            text: l3.copy_code.to_string(),
                            icon: Shape::ClipboardDocument,
                            on_pressed: move |e: UiEvent<MouseData>| {
                                e.cancel_bubble();
                                copy_friend_id2()
                            }
                        }
                    },
                }
            }),
        }
    ))
}
