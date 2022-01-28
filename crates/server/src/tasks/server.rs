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
                C2S::Request(client_id, Request::Event(event)) =>
                    self.handle_event(client_id, event).await,
                _ => {}
            }
        }
    }

    async fn broadcast(&mut self, event: Event) {
        let mut events = self
            .db
            .channel(&event)
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
            .iter::<User>()
            .map(|(_, user)| user)
            .find(|user| user.name == name)
            .map(|user| user.clone().into());

        self.db[client_id].respond(Response::User(user)).await;
    }

    async fn handle_get_room(&mut self, client_id: ClientId, name: String) {
        let room = self
            .db
            .iter::<Room>()
            .map(|(_, room)| room)
            .find(|room| room.name == name)
            .map(|room| room.clone().into());

        self.db[client_id].respond(Response::Room(room)).await;
    }

    async fn handle_connect(&mut self, client_id: ClientId, name: String) {
        // TODO unique name!
        let user = User::new(client_id, name);
        let user_id = user.id;
        let user_name = user.name.clone();

        let _ = self.db.insert((user_id, user));
        let client = &mut self.db[client_id];

        client.set_user(user_id);
        client
            .respond(Response::Connected(rat::User {
                id:   user_id,
                name: user_name,
            }))
            .await
            .expect("to_client closed");

        self.broadcast(user_id.enter_world()).await;
    }

    async fn handle_event(&mut self, client_id: ClientId, event: Event) {
        match event.channel {
            Channel::World => {
                //
                match event.event_type {
                    EventType::Enter => unreachable!("Clients must not send Enter World events"),
                    EventType::Leave => unreachable!("Clients must not send Leave World events"),
                    EventType::Post { .. } => {}
                }
            }
            Channel::Room { room_id } => {
                //
                match event.event_type {
                    EventType::Enter => {}
                    EventType::Leave => {}
                    EventType::Post { .. } => {}
                }
            }
        }

        self.broadcast(event).await
    }
}
