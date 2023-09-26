use self::{discord::DiscordProtocol, irc::IRCProtocol, matrix::MatrixProtocol};
use enum_dispatch::enum_dispatch;

use super::PurpurAPI;

pub mod discord;
pub mod irc;
pub mod matrix;

#[enum_dispatch]
pub trait Protocol {
    fn connect(&mut self, api: PurpurAPI);
    fn disconnect(&mut self);
}

#[enum_dispatch(Protocol)]
pub enum BuiltinProtocols {
    DiscordProtocol,
    IRCProtocol,
    MatrixProtocol,
}
