use crate::utils::sounds;
use notify_rust::Notification;

// Implementation to create and push new notifications
#[allow(non_snake_case)]
pub fn PushNotification(title: String, content: String, notification_sound: String) {
    let summary = format!("Uplink - {}", title);
    let _n = Notification::new()
        .summary(summary.as_ref())
        .body(content.as_ref())
        .show();
    // Play different notification sound for usual notifications and for friend request
    let _friend_request_string = String::from("Friend Request");
    let _usual_notification = String::from("Usual Notification");

    #[allow(unreachable_patterns)]
    match notification_sound {
        _friend_request_string => sounds::Play(sounds::Sounds::FriendReq),
        _usual_notification => sounds::Play(sounds::Sounds::Notification),
    }
}
