use super::*;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

/// Handles [`ClientTask`]s.
#[derive(Debug)]
pub struct ServerTask {
    from_client: FromClient,
    db:          Db,
}

impl ServerTask {
    pub fn new(from_client: FromClient) -> Self {
        Self {
            from_client,
            db: Db::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.from_client.recv().await {
            match message {
                C2S::Accept(to_client) => self.handle_accept(to_client).await,
                C2S::Request(client_id, Request::GetUser(name)) =>
                    self.handle_get_user(client_id, name).await,
                C2S::Request(client_id, Request::GetRoom(name)) =>
                    self.handle_get_room(client_id, name).await,
                C2S::Request(client_id, Request::Connect(name)) =>
                    self.handle_connect(client_id, name).await,
                C2S::Request(client_id, Request::CreateRoom(name)) =>
                    self.handle_create_room(client_id, name).await,
                C2S::Request(client_id, Request::Event(event)) =>
                    self.handle_event(client_id, event).await,
                C2S::Request(client_id, Request::Shutdown) => self.handle_shutdown(client_id),
                C2S::Request(client_id, _) => self.error(client_id).await,
            }
        }
    }

    async fn error(&self, client_id: ClientId) {
        self.db[client_id].respond(Response::Error).await;
    }

    async fn broadcast(
        &self,
        clients: impl IntoIterator<Item = &Client>,
        event: Event<rat::User, rat::Room>,
    ) {
        let mut events = clients
            .into_iter()
            .map(|client| client.event(event.clone()))
            .collect::<FuturesUnordered<_>>();

        while events.next().await.is_some() {}
    }

    async fn handle_accept(&mut self, to_client: ToClient) {
        let client = Client::new(to_client);
        let client_id = client.id;

        let _ = self.db.insert((client_id, client));
        self.db[client_id]
            .accepted(client_id)
            .await
            .expect("to_client closed"); // TODO remove client from db
    }

    async fn handle_get_user(&mut self, client_id: ClientId, name: String) {
        let user = self
            .db
            .find::<User, _>(|user| user.name == name)
            .map(|user| user.clone().into());

        self.db[client_id].respond(Response::User(user)).await;
    }

    async fn handle_get_room(&mut self, client_id: ClientId, name: String) {
        let room = self
            .db
            .find::<Room, _>(|room| room.name == name)
            .map(|room| room.clone().into());

        self.db[client_id].respond(Response::Room(room)).await;
    }

    async fn handle_connect(&mut self, client_id: ClientId, name: String) {
        let client = &mut self.db[client_id];

        client.make_user(name);
        client.connected().await.expect("to_client closed");
    }

    async fn handle_create_room(&mut self, client_id: ClientId, name: String) {
        let room = self.db.find::<Room, _>(|room| room.name == name);
        let response = if room.is_none() {
            let room = Room::new(name);
            let _ = self.db.insert((room.id, room.clone()));

            Response::CreatedRoom(room.into())
        } else {
            Response::Error
        };

        self.db[client_id].respond(response).await;
    }

    async fn handle_event(&mut self, client_id: ClientId, event: Event<UserId, RoomId>) {
        let user_id = event.user;

        match event.channel {
            Channel::World => {
                let event = match event.event_type {
                    EventType::Enter =>
                        if self.db.enter_world(client_id, user_id).is_ok() {
                            Event::enter_world(self.db[user_id].clone().into())
                        } else {
                            return self.error(client_id).await;
                        },
                    EventType::Leave =>
                        if let Ok(user) = self.db.leave_world(user_id) {
                            let event = Event::leave_world(user.into());
                            self.db[client_id].event(event.clone()).await;
                            event
                        } else {
                            return self.error(client_id).await;
                        },
                    EventType::Post { message } =>
                        if self.db.is_in_world(user_id) {
                            Event::post_world(self.db[user_id].clone().into(), message)
                        } else {
                            return self.error(client_id).await;
                        },
                };

                self.broadcast(self.db.world(), event).await;
            }
            Channel::Room(room_id) => {
                let event = match event.event_type {
                    EventType::Enter =>
                        if self.db.enter_room(user_id, room_id).is_ok() {
                            Event::enter_room(
                                self.db[user_id].clone().into(),
                                self.db[room_id].clone().into(),
                            )
                        } else {
                            return self.error(client_id).await;
                        },
                    EventType::Leave =>
                        if self.db.leave_room(user_id, room_id).is_ok() {
                            let event = Event::leave_room(
                                self.db[user_id].clone().into(),
                                self.db[room_id].clone().into(),
                            );
                            self.db[client_id].event(event.clone()).await;
                            event
                        } else {
                            return self.error(client_id).await;
                        },
                    EventType::Post { message } =>
                        if self.db.is_in_room(user_id, room_id) == Ok(true) {
                            Event::post_room(
                                self.db[user_id].clone().into(),
                                self.db[room_id].clone().into(),
                                message,
                            )
                        } else {
                            return self.error(client_id).await;
                        },
                };

                self.broadcast(self.db.room(room_id), event).await;
            }
        }
    }

    fn handle_shutdown(&mut self, client_id: ClientId) {
        if let Some(client) = self.db.remove(&client_id) {
            if let Some(user) = client.user {
                self.db.leave_world(user.id);
            }
        }
    }
}
