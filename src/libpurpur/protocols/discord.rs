use crate::libpurpur::UIAction;
use discord::Discord;

use super::Protocol;

pub struct DiscordProtocol {}

impl Protocol for DiscordProtocol {
    fn connect(&mut self, api: crate::libpurpur::PurpurAPI) {
        let client = Discord::from_user_token(std::env::var("DISCORD_TOKEN").unwrap().as_str()).unwrap();
        let (conn, _) = client.connect().unwrap();
        let servers = client.get_servers().unwrap();
        let names = servers.iter().map(|x| x.name.clone());
        api.send_ui_action(UIAction::SetChannelList(names.collect()))
        .unwrap();
    }

    fn disconnect(&mut self) {}
}
