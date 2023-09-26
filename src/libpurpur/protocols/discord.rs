use crate::libpurpur::{
    structures::{
        channels::{Channel, ChannelPlacement, RenderStyle},
        image::Image,
    },
    UIAction,
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
            id: 0.to_string(),
            name: x.name.clone(),
            image: Image::Url("https://picsum.photos/200".into()),
            children: None,
            preferred_render_style: RenderStyle::IconsAndText(false),
            placement: ChannelPlacement::Side(false),
        });
        api.send_ui_action(UIAction::SetChannelList(names.collect()))
            .unwrap();
    }

    fn disconnect(&mut self) {}
}
