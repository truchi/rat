use super::*;

/// A client.
#[derive(Clone, Debug)]
pub struct Client {
    pub id:        ClientId,
    pub user_id:   Option<UserId>,
    pub to_client: ToClient,
}

impl Client {
    pub fn new(to_client: ToClient) -> Self {
        Self {
            id: ClientId::new(),
            user_id: None,
            to_client,
        }
    }

    pub fn set_user(&mut self, user_id: UserId) {
        self.user_id = Some(user_id);
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
}
