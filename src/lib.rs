//! A dead simple notification daemon.

#![warn(missing_docs, clippy::unwrap_used)]

/// Error handler.
pub mod error;

/// D-Bus handler.
pub mod dbus;

/// Notification manager.
pub mod notification;

/// Rofi server
pub mod rofi;

use crate::dbus::DbusServer;
use crate::error::Result;
use log::debug;
use notification::Action;
use crate::rofi::RofiServer;
use notification::NotificationStore;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Service entry-point
pub fn run() -> Result<()> {
    let dbus_server = DbusServer::init()?;
    let timeout = Duration::from_millis(1000);

    let db = NotificationStore::init();
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        debug!("registering D-Bus server");
        dbus_server
            .register_notification_handler(sender, timeout)
            .expect("failed to register D-Bus notification handler");
    });

    let db_clone = db.clone();
    thread::spawn(|| {
        debug!("starting rofication server");
        let rofi_server = RofiServer::new("/tmp/rofi_notification_daemon".to_string(), db_clone);
        rofi_server.start().expect("Create domain socket server for rofication requests")
    });

    loop {
        match receiver.recv()? {
            Action::Show(notification) => {
                debug!("received notification: {} {}", notification.id, notification.body);                
                db.add(notification);
            }
            Action::ShowLast => {
                debug!("showing the last notification");
            }
            Action::Close(id) => {
                if let Some(id) = id {
                    debug!("closing notification: {}", id);
                    db.delete(id);
                } 
            }
            Action::CloseAll => {
                debug!("closing all notifications");
                db.delete_all();
            }
        }
    }
}
