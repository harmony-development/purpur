use gtk::glib::Sender;
use thiserror::Error;

use self::structures::{channels::{Channel, RenderStyle, ChannelPlacement}, Identifier, image::Image, notification::Notification};

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
pub enum Update {
    /// A new channel has been loaded/created.
    /// This update is guaranteed to come before the application has its ID.
    /// Changes to the channel will be reported through separate updates.
    NewChannel(Channel),
    ChannelName(Identifier<Channel>, String),
    ChannelImage(Identifier<Channel>, Image),
    ChannelChildren(Identifier<Channel>, Vec<Identifier<Channel>>),
    ChannelPreferredRenderStyle(Identifier<Channel>, RenderStyle),
    ChannelPlacement(Identifier<Channel>, ChannelPlacement),
    Notification(Notification),
    NewMessage(String),
}

#[derive(Debug, Clone)]
pub enum Query {
    DismissNotification(Identifier<Notification>),
}

#[derive(Error, Debug)]
pub enum SDKError {
    #[error("failed to send ui action")]
    SendFailure,
}

pub struct PurpurAPI {
    pub update_sender: Sender<Update>,
}

impl PurpurAPI {
    pub fn send_update(&self, action: Update) -> Result<(), SDKError> {
        self.update_sender
            .send(action)
            .map_err(|_| SDKError::SendFailure)
    }
}
