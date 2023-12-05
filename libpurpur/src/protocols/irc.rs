use crate::{
    structures::{
        channels::{Channel, ChannelPlacement, RenderStyle},
        Identifier,
    },
    PurpurAPI, Update,
};
use futures::StreamExt;
use irc::client::prelude::*;
use tokio::runtime::Runtime;

use super::Protocol;

pub struct IRCProtocol {}

impl IRCProtocol {
    pub fn new() -> IRCProtocol {
        IRCProtocol {}
    }
}

impl Protocol for IRCProtocol {
    fn connect(&mut self, api: PurpurAPI) {
        let rt = Runtime::new().unwrap();
        let channels = vec![
            "#meta".to_string(),
            "#gemini".to_string(),
            "#rust-spam".to_string(),
        ];

        rt.block_on(async {
            let mut client = Client::from_config(Config {
                nickname: Some("silicat".into()),
                server: Some("tilde.chat".into()),
                channels: channels.clone(),
                ..Config::default()
            })
            .await
            .unwrap();
            client.identify().unwrap();
            let mut stream = client.stream().unwrap();
            for x in channels {
                api.send_update(Update::NewChannel(Channel {
                    id: Identifier::new(x.clone()),
                    name: x,
                    image: None,
                    children: None,
                    preferred_render_style: RenderStyle::TextOnly,
                    placement: ChannelPlacement::Under,
                }))
                .await
                .unwrap();
            }
            while let Some(message) = stream.next().await.transpose().unwrap() {
                api.send_update(Update::NewMessage(message.to_string()))
                    .await
                    .unwrap();
            }
        });
    }

    fn disconnect(&mut self) {}

    fn query(&mut self, query: crate::Query) {
        todo!()
    }
}
