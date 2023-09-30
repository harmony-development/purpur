use self::{discord::DiscordProtocol, irc::IRCProtocol, matrix::MatrixProtocol};
use enum_dispatch::enum_dispatch;

use super::{PurpurAPI, Query};

pub mod discord;
pub mod irc;
pub mod matrix;

#[enum_dispatch]
pub trait Protocol {
    fn connect(&mut self, api: PurpurAPI);
    fn query(&mut self, query: Query);
    fn disconnect(&mut self);
}

#[enum_dispatch(Protocol)]
pub enum BuiltinProtocols {
    DiscordProtocol,
    IRCProtocol,
    MatrixProtocol,
}
