use crate::libpurpur::{
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

impl Protocol for DiscordProtocol {
    fn connect(&mut self, api: crate::libpurpur::PurpurAPI) {
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
        });
        names.for_each(|x| {
            api.send_update(Update::NewChannel(x));
        });
    }

    fn disconnect(&mut self) {}

    fn query(&mut self, query: crate::libpurpur::Query) {
        todo!()
    }
}
