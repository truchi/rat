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
                _ => {}
            }
        }
    }

    async fn broadcast(&mut self, event: Event<UserId, RoomId>) {
        let user = self.db[event.user].clone().into();
        let event = match event.channel {
            Channel::World => Event {
                channel: Channel::World,
                user,
                event_type: event.event_type,
            },
            Channel::Room(room_id) => Event {
                channel: Channel::Room(self.db[room_id].clone().into()),
                user,
                event_type: event.event_type,
            },
        };

        let mut events = self
            .db
            .world()
            .map(|client| client.respond(Response::Event(event.clone())))
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
            Channel::World => match event.event_type {
                EventType::Enter =>
                    if self.db.enter_world(client_id).is_err() {
                        self.db[client_id].respond(Response::Error).await;
                        return;
                    },
                EventType::Leave =>
                    if self.db.leave_world(user_id).is_err() {
                        self.db[client_id].respond(Response::Error).await;
                        return;
                    },
                _ =>
                    if !self.db.is_in_world(user_id) {
                        self.db[client_id].respond(Response::Error).await;
                        return;
                    },
            },
            Channel::Room(room_id) => match event.event_type {
                EventType::Enter =>
                    if self.db.enter(user_id, room_id).is_err() {
                        self.db[client_id].respond(Response::Error).await;
                        return;
                    },
                EventType::Leave =>
                    if self.db.leave(user_id, room_id).is_err() {
                        self.db[client_id].respond(Response::Error).await;
                        return;
                    },
                _ =>
                    if self.db.is_in(user_id, room_id) != Ok(true) {
                        self.db[client_id].respond(Response::Error).await;
                        return;
                    },
            },
        }

        self.broadcast(event).await
    }
}
