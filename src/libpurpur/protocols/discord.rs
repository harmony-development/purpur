use crate::libpurpur::UIAction;

use super::Protocol;

pub struct Discord {}

impl Protocol for Discord {
    fn connect(&mut self, api: crate::libpurpur::PurpurAPI) {
        api.send_ui_action(UIAction::SetChannelList(vec!["meow".to_string(), "blehh".to_string()])).unwrap();
    }

    fn disconnect(&mut self) {}
}
