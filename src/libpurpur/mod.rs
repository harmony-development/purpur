use gtk::glib::Sender;
use thiserror::Error;

pub mod protocols;
pub mod structures;

#[derive(Debug)]
pub enum UIAction {
    SetChannelList(Vec<String>),
    NewMessage(String)
}

#[derive(Error, Debug)]
pub enum SDKError {
    #[error("failed to send ui action")]
    SendFailure
}

pub struct PurpurAPI {
    pub action_sender: Sender<UIAction>,
}

impl PurpurAPI {
    pub fn send_ui_action(&self, action: UIAction) -> Result<(), SDKError> {
        self.action_sender.send(action).map_err(|_| SDKError::SendFailure)
    }
}
