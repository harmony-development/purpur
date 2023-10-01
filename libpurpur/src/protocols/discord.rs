use crate::{
    structures::{
        channels::{Channel, ChannelPlacement, RenderStyle},
        image::Image,
        Identifier,
    },
    Update,
};
use discord::Discord;

use super::Protocol;

pub struct DiscordProtocol {}

impl DiscordProtocol {
    pub fn new() -> DiscordProtocol {
        DiscordProtocol { }
    }
}

impl Protocol for DiscordProtocol {
    fn connect(&mut self, api: crate::PurpurAPI) {
        let client =
            Discord::from_user_token(std::env::var("DISCORD_TOKEN").unwrap().as_str()).unwrap();
        let (conn, _) = client.connect().unwrap();
        let servers = client.get_servers().unwrap();
        let names = servers.iter().map(|x| Channel {
            id: Identifier::new(0.to_string()),
            name: x.name.clone(),
            image: Image::Url("https://picsum.photos/200".into()),
            children: None,
            preferred_render_style: RenderStyle::IconsAndText(false),
            placement: ChannelPlacement::Side(false),
        }).collect::<Vec<Channel>>();
        tokio::spawn(async move {
            for name in names {
                api.send_update(Update::NewChannel(name)).await.unwrap();
            }
        });
    }

    fn disconnect(&mut self) {}

    fn query(&mut self, query: crate::Query) {
        todo!()
    }
}
