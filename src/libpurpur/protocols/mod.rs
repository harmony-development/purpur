use self::discord::Discord;
use enum_dispatch::enum_dispatch;

use super::PurpurAPI;

pub mod discord;

#[enum_dispatch]
pub trait Protocol {
    fn connect(&mut self, api: PurpurAPI);
    fn disconnect(&mut self);
}

#[enum_dispatch(Protocol)]
pub enum BuiltinProtocols {
    Discord
}
