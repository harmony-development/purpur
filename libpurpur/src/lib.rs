use thiserror::Error;
use tokio::sync::mpsc::Sender;

use self::structures::{
    channels::{Channel, ChannelPlacement, RenderStyle},
    image::Image,
    notification::Notification,
    Identifier,
};

pub mod protocols;
pub mod structures;

#[derive(Debug, Clone)]
pub enum NotificationLevel {
    Debug,
    Info,
    Warning,
    Error,
}

const _: () = {
    fn assert_type<T: Send + Sync>() {}

    fn assert() {
        assert_type::<Update>();
        assert_type::<Query>();
    }
};

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

#[derive(Clone)]
pub struct PurpurAPI {
    update_sender: Sender<Update>,
}

impl PurpurAPI {
    pub async fn send_update(&self, action: Update) -> Result<(), SDKError> {
        self.update_sender.send(action).await.map_err(|_| SDKError::SendFailure)
    }
    pub fn new(with: Sender<Update>) -> PurpurAPI {
        PurpurAPI { update_sender: with }
    }
}
