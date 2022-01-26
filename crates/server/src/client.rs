use super::*;

pub struct Client {
    id:           Uuid,
    stream:       TcpStream,
    to_manager:   ToManager,
    from_manager: FromManager,
}

impl Client {
    pub async fn new(stream: TcpStream, to_manager: ToManager) -> Result<Self, TcpStream> {
        let (to_client, mut from_manager) = mpsc::channel(32);

        if to_manager.send(Accept(to_client)).await.is_ok() {
            if let Some(Accepted(id)) = from_manager.recv().await {
                return Ok(Self {
                    id,
                    stream,
                    to_manager,
                    from_manager,
                });
            }
        }

        Err(stream)
    }

    pub async fn run(&mut self) {
        loop {
            let request = self.stream.recv();
            let response = self.from_manager.recv();

            select! {
                request = request => self.handle_request(request).await,
                Some(Response(response)) = response => self.handle_response(response).await,
                else => break,
            }
        }
    }

    async fn handle_request(&mut self, request: ClientRequest) {
        self.to_manager
            .send(Request(self.id, request))
            .await
            .expect("to_manager closed");
    }

    async fn handle_response(&mut self, response: ServerResponse) {
        self.stream.send(&response).await;
    }
}
