use super::*;

/// A client.
#[derive(Clone, Debug)]
pub struct Client {
    pub id:        ClientId,
    pub user:      Option<User>,
    pub to_client: ToClient,
}

impl Client {
    pub fn new(to_client: ToClient) -> Self {
        Self {
            id: ClientId::new(),
            user: None,
            to_client,
        }
    }

    pub fn make_user(&mut self, name: String) {
        self.user = Some(User::new(self.id, name));
    }

    pub async fn send(&self, message: ServerToClient) -> Result<(), ()> {
        self.to_client.send(message).await.map_err(|_| ())
    }

    pub async fn accepted(&self, client_id: ClientId) -> Result<(), ()> {
        self.send(S2C::Accepted(client_id)).await
    }

    pub async fn respond(&self, response: Response) -> Result<(), ()> {
        self.send(S2C::Response(response)).await
    }

    pub async fn connected(&self) -> Result<(), ()> {
        self.respond(Response::Connected(self.user.clone().unwrap().into()))
            .await
    }
}
