use dioxus::prelude::*;
use ui_kit::button::Button;
use utils::Account;
use warp::crypto::DID;

#[derive(Props, PartialEq)]
pub struct MoreMenuProps {
    account: Account,
    friend: DID,
}

#[allow(non_snake_case)]
pub fn MoreMenu(cx: Scope<MoreMenuProps>) -> Element {
    let friend_id = &cx.props.friend.to_string()[8..];
    let script = include_str!("./more_menu.js").replace("friend_id", &friend_id);

    let more_memu = rsx!(
            Button {
                        text:"Remove Friend".to_string(),
                        state: ui_kit::button::State::Transparent,
                        on_pressed: move |_| {
                                let mut multipass = cx.props.account.clone();
                                let did_to_remove = cx.props.friend.clone();
                                match multipass.remove_friend(&did_to_remove) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        log::debug!("error removing friend");
                                    }
                                }
                                //todo: remove the conversation?
                            }
                        },
            Button {
                            text:"Block Friend".to_string(),
                            state: ui_kit::button::State::Transparent,
                            on_pressed: move |_| {
                                 let mut multipass = cx.props.account.clone();
                                 let did_to_block = cx.props.friend.clone();
                                 match multipass.block(&did_to_block) {
                                     Ok(_) => {}
                                     Err(e) => {
                                         log::debug!("faied to block friend {}:{}", &cx.props.friend, e);
                                     }
                                 }
                             }
                    },
            script { "{script}"}
    );
    cx.render(more_memu)
}
