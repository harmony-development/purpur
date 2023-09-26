use gtk::glib::Sender;
use thiserror::Error;

use self::structures::channels::Channel;

pub mod protocols;
pub mod structures;

#[derive(Debug, Clone)]
pub enum NotificationLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub enum UIAction {
    SetChannelList(Vec<Channel>),
    NewMessage(String),
    /// Shows a notification with the text with a specified severity.
    /// Callback fires when its dismissed / closed.
    SendNotification {
        /// The severity of the notification
        level: NotificationLevel,
        /// The class of the notification, notifications of a certain nature should be given the
        /// same class, for example "Login Failed" should produce a notification of class
        /// me.blusk.purpur-discord.login-failed
        class: String,
        /// Notification title
        title: String,
        /// Notification body
        body: String,
        cb: fn() -> (),
    },
}

#[derive(Error, Debug)]
pub enum SDKError {
    #[error("failed to send ui action")]
    SendFailure,
}

pub struct PurpurAPI {
    pub action_sender: Sender<UIAction>,
}

impl PurpurAPI {
    pub fn send_ui_action(&self, action: UIAction) -> Result<(), SDKError> {
        self.action_sender
            .send(action)
            .map_err(|_| SDKError::SendFailure)
    }
}
