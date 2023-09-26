use std::env;

use super::Protocol;
use crate::libpurpur::PurpurAPI;
use anyhow::Context;
use futures::StreamExt;
use matrix_sdk::{
    config::SyncSettings,
    ruma::{api::client::space, user_id},
    Client,
};
use matrix_sdk_ui::{room_list_service::RoomList, sync_service::State};
use tokio::runtime::Runtime;

pub struct MatrixProtocol {}

impl Protocol for MatrixProtocol {
    fn connect(&mut self, api: PurpurAPI) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let account = user_id!("@blusk:matrix.blusk.dev");
            let client = Client::builder()
                .server_name(account.server_name())
                .build()
                .await?;
            client
                .matrix_auth()
                .login_username(account, &env::var("MATRIX_PASSWORD").unwrap())
                .send()
                .await?;
            let sync_service = matrix_sdk_ui::sync_service::SyncService::builder(client)
                .build()
                .await?;

            sync_service.start().await;

            let mut sync_service_state = sync_service.state();

            tokio::spawn(async move {
                if let Some(state) = sync_service_state.next().await {
                    match state {
                        State::Terminated => {
                            println!("The process has been terminated.");
                        }
                        State::Idle => {
                            println!("The system is currently idle.");
                        }
                        State::Running => {
                            println!("The system is currently running.");
                        }
                        State::Error => {
                            println!("An error has occurred.");
                        }
                    }
                }
            });

            let (entries, subscriber) = sync_service
                .room_list_service()
                .all_rooms()
                .await?
                .entries();
            println!("entries {:?}", entries);
            subscriber
                .for_each(|x| async move {
                    println!("hi {:?}", x);
                })
                .await;

            anyhow::Ok(())
        })
        .ok();
    }

    fn disconnect(&mut self) {
        todo!()
    }
}
