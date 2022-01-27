use super::*;

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
                Accept(to_client) => self.handle_accept(to_client).await,
                Request(id, ConnectUser { name }) => self.handle_connect_user(id, name).await,
                _ => {}
            }
        }
    }

    async fn handle_accept(&mut self, to_client: ToClient) {
        let client = Client::new(to_client);
        let id = client.id;

        let _ = self.db.insert((id, client));
        self.db
            .get_mut(&id)
            .expect("just inserted id")
            .accepted(id)
            .await
            .expect("to_client closed"); // TODO remove client from db
    }

    async fn handle_connect_user(&mut self, id: ClientId, name: String) {
        /*
        self.world.get_mut(&id).expect("Cannot find client").user = Some(user.clone());
        let _ = self
            .world
            .insert((user.clone(), UserData::new(id, user.name.clone())));

        self.world
            .get_mut(&id)
            .expect("Cannot find client")
            .respond(ConnectedUser(user))
            .await
            .expect("to_client closed");

        // TODO broacast world Enter(World, user)
        */
    }

    async fn broacast<T: Iterator<Item = ClientId>>(&mut self, clients: T, event: Event) {}
}
