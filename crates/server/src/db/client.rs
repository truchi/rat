use super::*;

/// A client.
#[derive(Debug)]
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

    pub async fn send(&mut self, message: ServerToClient) -> Result<(), ()> {
        self.to_client.send(message).await.map_err(|_| ())
    }

    pub async fn accepted(&mut self, client_id: ClientId) -> Result<(), ()> {
        self.send(Accepted(client_id)).await
    }

    pub async fn respond(&mut self, response: ServerResponse) -> Result<(), ()> {
        self.send(Response(response)).await
    }

    pub async fn event(&mut self, event: Event) -> Result<(), ()> {
        self.respond(Evented(event)).await
    }
}
