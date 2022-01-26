use super::*;

#[derive(Debug)]
pub struct Manager {
    from_client: FromClient,
    world:       World,
}

impl Manager {
    pub fn new(from_client: FromClient) -> Self {
        Self {
            from_client,
            world: World::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.from_client.recv().await {
            match message {
                Accept(to_client) => self.handle_accept(to_client).await,
                Request(id, ConnectUser(user)) => self.handle_connect_user(id, user).await,
                _ => {}
            }
        }
    }

    async fn handle_accept(&mut self, to_client: ToClient) {
        let client = ClientData::new(to_client);
        let id = client.id;

        let _ = self.world.insert((id, client));
        self.world
            .get_mut(&id)
            .expect("just inserted id")
            .accepted(id)
            .await
            .expect("to_client closed"); // TODO remove client from world
    }

    async fn handle_connect_user(&mut self, id: Uuid, user: User) {
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
    }

    async fn broacast<T: Iterator<Item = Uuid>>(&mut self, clients: T, event: Event) {}
}
