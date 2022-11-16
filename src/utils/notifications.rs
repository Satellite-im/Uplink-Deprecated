use crate::utils::sounds::{Play, Sounds};
use notify_rust::Notification;

// Implementation to create and push new notifications

#[allow(non_snake_case)]
pub fn PushNotification(title: String, content: String, notification_sound: Sounds) {
    let summary = format!("Uplink - {}", title);
    let _n = Notification::new()
        .summary(summary.as_ref())
        .body(&content)
        .show();

    Play(notification_sound)
}
