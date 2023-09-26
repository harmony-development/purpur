use crate::libpurpur::{PurpurAPI, UIAction};
use futures::StreamExt;
use irc::client::prelude::*;
use tokio::runtime::Runtime;

use super::Protocol;

pub struct IRCProtocol {}

impl Protocol for IRCProtocol {
    fn connect(&mut self, api: PurpurAPI) {
        println!("meow1");
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            println!("meow");
            let mut client = Client::from_config(Config {
                nickname: Some("silicat".into()),
                server: Some("tilde.chat".into()),
                channels: vec![
                    "#meta".to_owned(),
                    "#gemini".to_owned(),
                    "#rust-spam".to_owned(),
                ],
                ..Config::default()
            })
            .await
            .unwrap();
            client.identify().unwrap();
            let mut stream = client.stream().unwrap();
            while let Some(message) = stream.next().await.transpose().unwrap() {
                api.send_ui_action(UIAction::NewMessage(message.to_string()))
                    .unwrap();
            }
        });
    }

    fn disconnect(&mut self) {}
}
