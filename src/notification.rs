use serde::Serialize;
use serde_repr::Serialize_repr;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

/// Name of the template for rendering the notification message.
pub const NOTIFICATION_MESSAGE_TEMPLATE: &str = "notification_message_template";

/// Possible urgency levels for the notification.
#[derive(Clone, Debug, Serialize_repr, Copy)]
#[repr(u8)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl Display for Urgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

impl From<u64> for Urgency {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Normal,
            2 => Self::Critical,
            _ => Self::default(),
        }
    }
}

impl Default for Urgency {
    fn default() -> Self {
        Self::Normal
    }
}
/// Representation of a notification.
///
/// See [D-Bus Notify Parameters](https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html)
#[derive(Clone, Debug, Default, Serialize)]
pub struct Notification {
    pub id: u32,
    pub summary: String,
    pub body: String,
    pub application: String,
    pub icon: String,
    pub urgency: Urgency,
    pub actions: Vec<String>,
    pub hints: HashMap<String, String>,
    pub timestamp: u64,
}

/// Possible actions for a notification.
#[derive(Debug)]
pub enum Action {
    /// Show a notification.
    Show(Notification),
    /// Show the last notification.
    ShowLast,
    /// Close a notification.
    Close(Option<u32>),
    /// Close all the notifications.
    CloseAll,
}

/// Notification manager.
#[derive(Debug)]
pub struct NotificationStore {
    /// Inner type that holds the notifications in thread-safe way.
    inner: Arc<RwLock<Vec<Notification>>>,
}

impl Clone for NotificationStore {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl NotificationStore {
    /// Initializes the notification manager.
    pub fn init() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Returns the number of notifications.
    pub fn count(&self) -> usize {
        self.inner
            .read()
            .expect("failed to retrieve notifications")
            .len()
    }

    /// Adds a new notifications to manage.
    pub fn add(&self, notification: Notification) {
        self.inner
            .write()
            .expect("failed to retrieve notifications")
            .push(notification);
    }

    pub fn items(&self) -> Vec<Notification> {
        let readable_clone = self.inner.read().expect("can read").clone();

        let mut rv: Vec<Notification> = vec!();

        for n in readable_clone {
            rv.push(n.clone());
        }

        return rv;
    }

    /// Marks the given notification as read.
    pub fn mark_as_read(&self, id: u32) {
        let mut notifications = self
            .inner
            .write()
            .expect("failed to retrieve notifications");
        if let Some(_notification) = notifications
            .iter_mut()
            .find(|notification| notification.id == id)
        {
            println!("mark_as_read() called but NOP")
            // FIXME(kggilmer)
            //notification.is_read = true;
        }
    }
    /// Marks all the notifications as read.
    pub fn mark_all_as_read(&self) {
        let mut notifications = self
            .inner
            .write()
            .expect("failed to retrieve notifications");
        notifications.clear()
    }
    /// Returns the number of unread notifications.
    pub fn get_unread_count(&self) -> usize {
        let notifications = self.inner.read().expect("failed to retrieve notifications");
        notifications.iter().count()
    }
}
