use arboard::Clipboard;
use dioxus::{
    core::UiEvent,
    events::{FormEvent, MouseData},
    prelude::*,
};
use dioxus_heroicons::outline::Shape;
use dioxus_toast::{Position, ToastInfo};

use crate::{Account, LANGUAGE, TOAST_MANAGER};

use ui_kit::{button::Button, input::Input};

use warp::crypto::DID;

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
    let code2 = { l.code_copied.to_string() };
    let account2 = account.clone();

    let copy_friend_id = move || {
        let mut clipboard = Clipboard::new().unwrap();
        if let Ok(ident) = account2.get_own_identity() {
            let single_toast = ToastInfo {
                position: Position::TopRight,
                ..ToastInfo::simple(&code2)
            };
            let _id = toast.write().popup(single_toast); // copy to the clipboard without prefix 'did:key:'
            clipboard
                .set_text(&ident.did_key().to_string()[8..])
                .unwrap();
        }
    };
    let copy_friend_id2 = copy_friend_id.clone();

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
										icon: Shape::UserPlus,
										on_change: move |evt: FormEvent| {
												add_error.set(String::new());
												remote_friend.set(evt.value.clone());
										},
										on_enter: move |_| {
														let did = DID::try_from(format!("did:key:{}", remote_friend.clone()));
												match did {
														Ok(d) => {
																match account.clone()
																		.send_request(&d)
																{
																		Ok(_) => {
																				let single_toast = ToastInfo {
																						position: Position::TopRight,
																						..ToastInfo::simple(l.request_sent.clone().as_ref())
																				};
																				let _id = toast.write().popup(single_toast);
																				add_error.set("".into());
																		}
																		Err(e) => {
																				add_error.set(match e {
																						warp::error::Error::CannotSendFriendRequest => l.couldnt_send.to_string(),
																						warp::error::Error::FriendRequestExist => l.already_sent.to_string(),
																						warp::error::Error::CannotSendSelfFriendRequest => l.add_self.clone(),
																						warp::error::Error::FriendExist => l.friend_exist.to_string(),
																						_ => l.something_went_wrong.to_string()
																				})
																		},
																};
														},
														Err(_) => add_error.set(l.invalid_code.clone()),
												}
										}
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
						}),
				}
    ))
}
