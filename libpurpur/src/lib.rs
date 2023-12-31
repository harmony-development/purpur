use std::thread;

use protocols::Protocol;
use structures::widget::Dialog;
use thiserror::Error;
use kanal::{AsyncReceiver, AsyncSender};
use serde::{Serialize, Deserialize};

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
        assert_type::<PurpurAPI>();
        assert_type::<Purpur>();
    }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    NewMessage(String),
    Notification(Notification),
    ShowDialog(Dialog),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    update_sender: AsyncSender<Update>,
}

impl PurpurAPI {
    pub async fn send_update(&self, action: Update) -> Result<(), SDKError> {
        self.update_sender.send(action).await.map_err(|_| SDKError::SendFailure)
    }
}

#[derive(Clone)]
pub struct Purpur {
    api: PurpurAPI,
    pub update_receiver: AsyncReceiver<Update>,
}

impl Purpur {
    pub fn new() -> Purpur {
        let (update_send, update_read) = kanal::bounded_async(32);
        Purpur {
            api: PurpurAPI {
                update_sender: update_send
            },
            update_receiver: update_read
        }
    }
    pub fn add_protocol(&self, mut protocol: Box<dyn Protocol + Send>) {
        let api = self.api.clone();
        thread::spawn(move || {
            protocol.connect(api);
        });
    }
}
