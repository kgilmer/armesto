//! A dead simple notification daemon.

#![warn(missing_docs, clippy::unwrap_used)]

/// Error handler.
pub mod error;

/// D-Bus handler.
pub mod dbus;

/// Notification manager.
pub mod notification;

pub mod rofi;

use crate::dbus::DbusServer;
use crate::error::Result;
use crate::notification::Action;
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

    let notifications = Manager::init();
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        println!("registering D-Bus handler");
        dbus_server
            .register_notification_handler(sender, timeout)
            .expect("failed to register D-Bus notification handler");
    });

    thread::spawn(move || {
        let rofi_server = RofiServer::new("/tmp/testsocket".to_string());
        rofi_server.start();
    });

    loop {
        match receiver.recv()? {
            Action::Show(notification) => {
                println!("received notification: {} {}", notification.id, notification.body);
                notifications.add(notification);
            }
            Action::ShowLast => {
                println!("showing the last notification");
            }
            Action::Close(id) => {
                if let Some(id) = id {
                    println!("closing notification: {}", id);
                    notifications.mark_as_read(id);
                } else {
                    println!("closing the last notification");
                    notifications.mark_last_as_read();
                }
            }
            Action::CloseAll => {
                println!("closing all notifications");
                notifications.mark_all_as_read();
            }
        }
    }
}
