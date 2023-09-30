use std::{env, sync::Arc};

use super::Protocol;
use crate::PurpurAPI;
use eyeball_im::VectorDiff;
use futures::StreamExt;
use imbl::{HashMap, HashSet};
use matrix_sdk::{
    ruma::{user_id::UserId, OwnedRoomId, RoomId},
    Client, RoomListEntry,
};
use matrix_sdk_ui::{
    sync_service::{State, SyncService},
    timeline::TimelineItem,
    RoomListService,
};
use tokio::runtime::Runtime;
use tracing::{debug, error, info, trace, Instrument};

enum RoomTrackingState {}

type RoomStateChangeStream = kanal::AsyncReceiver<RoomTrackingState>;

#[derive(Clone)]
struct Connected {
    sync_service: Arc<SyncService>,
    room_list_service: Arc<RoomListService>,
    api: PurpurAPI,
}

impl Connected {
    async fn track_room(&self, room_id: &RoomId) -> RoomStateChangeStream {
        let api = self.api.clone();
        let room = self.room_list_service.room(room_id).await.unwrap();
        let (mut current_events, mut sub) = room.timeline().await.subscribe().await;
        let (sender, receiver) = kanal::unbounded_async();
        let task = async move {
            debug!("tracking room");
            // TODO(dusk): this is just temporary, we'll want to write separate handles for "real" events and virtual events
            // and also handle pushbacks / pushfronts and set etc. properly
            let handle_event = |event: &TimelineItem| {
                // TODO(dusk): this is just here for now so we can see every message that is sent
                if let Some(message) = event.as_event().and_then(|v| v.content().as_message()) {
                    api.send_update(crate::Update::NewMessage(
                        message.body().to_owned(),
                    ))
                    .unwrap();
                }
            };
            // handle diffs
            while let Some(diff) = sub.next().await {
                debug!("diff: {:?}", diff);
                match diff {
                    // TODO(dusk): i dont think this actually gets used?
                    VectorDiff::Append { values } => {
                        for event in values.iter() {
                            handle_event(event);
                        }
                        current_events.append(values);
                    }
                    VectorDiff::Clear => {
                        current_events.clear();
                    }
                    VectorDiff::Set { index, value } => {
                        current_events.set(index, value);
                    }
                    VectorDiff::PushBack { value } => {
                        handle_event(&value);
                        current_events.push_back(value);
                    }
                    VectorDiff::PushFront { value } => {
                        handle_event(&value);
                        current_events.push_front(value);
                    }
                    VectorDiff::Remove { index } => {
                        current_events.remove(index);
                    }
                    x => {
                        error!("unimplemented room diff: {x:?}");
                        sender.close();
                        break;
                    }
                }
            }
        };
        tokio::spawn(task.instrument(tracing::info_span!("room tracking", room_id = %room_id)));
        receiver
    }
}

pub struct MatrixProtocol {
    connected: Option<Connected>,
}

impl MatrixProtocol {
    pub fn new() -> Self {
        Self { connected: None }
    }

    fn room_list_service(&self) -> &Arc<RoomListService> {
        self.connected
            .as_ref()
            .map(|c| &c.room_list_service)
            .expect("not connected")
    }

    fn sync_service(&self) -> &Arc<SyncService> {
        self.connected
            .as_ref()
            .map(|c| &c.sync_service)
            .expect("not connected")
    }

    fn api(&self) -> &PurpurAPI {
        self.connected
            .as_ref()
            .map(|c| &c.api)
            .expect("not connected")
    }

    fn connected(&self) -> Connected {
        self.connected.clone().expect("not connected")
    }
}

impl Protocol for MatrixProtocol {
    fn connect(&mut self, api: PurpurAPI) {
        let rt = Runtime::new().unwrap();
        let task = async move {
            let account = UserId::parse(env::var("MATRIX_USER").unwrap()).unwrap();
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
            let sync_state_task = async move {
                if let Some(state) = sync_service_state.next().await {
                    match state {
                        State::Terminated => {
                            info!("The process has been terminated.");
                        }
                        State::Idle => {
                            debug!("The system is currently idle.");
                        }
                        State::Running => {
                            debug!("The system is currently running.");
                        }
                        State::Error => {
                            error!("An error has occurred.");
                        }
                    }
                }
            };
            tokio::spawn(sync_state_task.in_current_span());

            self.connected = Some(Connected {
                room_list_service: sync_service.room_list_service(),
                sync_service: Arc::new(sync_service),
                api,
            });
            info!("connected to matrix");

            let (mut room_list, mut subscriber) =
                self.room_list_service().all_rooms().await?.entries();

            let connected = self.connected();
            let rooms_task = async move {
                let mut rooms_being_tracked: HashMap<OwnedRoomId, RoomStateChangeStream> =
                    HashMap::new();
                while let Some(entries) = subscriber.next().await {
                    // remove rooms if we stop tracking them
                    let mut rooms_to_stop_tracking = Vec::new();
                    for room_id in rooms_being_tracked.keys() {
                        if rooms_being_tracked.get(room_id).unwrap().is_closed() {
                            rooms_to_stop_tracking.push(room_id.clone());
                        }
                    }
                    for room_id in rooms_to_stop_tracking {
                        debug!("stopped tracking room: {room_id}");
                        rooms_being_tracked.remove(&room_id);
                    }
                    // process diffs
                    for diff in entries {
                        match diff {
                            VectorDiff::Append { values } => {
                                room_list.append(values);
                            }
                            VectorDiff::Set { index, value } => {
                                if let RoomListEntry::Filled(id) = &value {
                                    if !rooms_being_tracked.contains_key(id) {
                                        let stream =
                                            connected.track_room(id).in_current_span().await;
                                        rooms_being_tracked.insert(id.clone(), stream);
                                    }
                                }
                                room_list.set(index, value);
                            }
                            _ => todo!(),
                        }
                    }
                }
            };
            let rooms_handle = tokio::spawn(rooms_task.in_current_span());
            rooms_handle.await?;

            anyhow::Ok(())
        };
        let result = rt.block_on(task.instrument(tracing::info_span!("matrix")));
        if let Err(err) = result {
            error!("matrix error: {err}");
        }
    }

    fn disconnect(&mut self) {
        todo!()
    }

    fn query(&mut self, query: crate::Query) {
        todo!()
    }
}
