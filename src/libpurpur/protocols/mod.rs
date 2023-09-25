use self::{discord::DiscordProtocol, irc::IRCProtocol};
use enum_dispatch::enum_dispatch;

use super::PurpurAPI;

pub mod discord;
pub mod irc;

#[enum_dispatch]
pub trait Protocol {
    fn connect(&mut self, api: PurpurAPI);
    fn disconnect(&mut self);
}

#[enum_dispatch(Protocol)]
pub enum BuiltinProtocols {
    DiscordProtocol,
    IRCProtocol,
}
