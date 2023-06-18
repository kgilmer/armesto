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
use notification::Action;
use crate::rofi::RofiServer;
use notification::Manager;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Runs `armesto`.
pub fn run() -> Result<()> {
    tracing::info!("starting");

    let dbus_server = DbusServer::init()?;
    let timeout = Duration::from_millis(1000);

    let db = Manager::init();
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        println!("registering D-Bus handler");
        dbus_server
            .register_notification_handler(sender, timeout)
            .expect("failed to register D-Bus notification handler");
    });

    let db_copy = db.clone();
    thread::spawn(|| {
        println!("starting rofi server");
        let rofi_server = RofiServer::new("/tmp/testsocket".to_string(), db_copy);
        rofi_server.start().expect("Create domain socket server for rofication requests")
    });

    loop {
        match receiver.recv()? {
            Action::Show(notification) => {
                println!("received notification: {} {}", notification.id, notification.body);
                db.add(notification);
            }
            Action::ShowLast => {
                println!("showing the last notification");
            }
            Action::Close(id) => {
                if let Some(id) = id {
                    println!("closing notification: {}", id);
                    db.mark_as_read(id);
                } else {
                    println!("closing the last notification");
                    db.mark_last_as_read();
                }
            }
            Action::CloseAll => {
                println!("closing all notifications");
                db.mark_all_as_read();
            }
        }
    }
}
