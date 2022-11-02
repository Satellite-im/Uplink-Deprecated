use notify_rust::Notification;

pub struct Notifications {}

// Implementation to create and push new notifications
impl Notifications {
    pub fn push(title: String, content: String) {
        let summary = format!("Uplink - {}", title);
        let _n = Notification::new()
            .summary(summary.as_ref())
            .body(content.as_ref())
            .show();
    }
}
