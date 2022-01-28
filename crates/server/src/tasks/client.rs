use super::*;

/// A task interfacing between a [`Client`](db::Client)
/// and the [`ServerTask`].
pub struct ClientTask {
    id:          ClientId,
    stream:      TcpStream,
    to_server:   ToServer,
    from_server: FromServer,
}

impl ClientTask {
    pub async fn new(stream: TcpStream, to_server: ToServer) -> Result<Self, TcpStream> {
        let (to_client, mut from_server) = mpsc::channel(32);

        if to_server.send(C2S::Accept(to_client)).await.is_ok() {
            if let Some(S2C::Accepted(id)) = from_server.recv().await {
                return Ok(Self {
                    id,
                    stream,
                    to_server,
                    from_server,
                });
            }
        }

        Err(stream)
    }

    pub async fn run(&mut self) {
        loop {
            let request = self.stream.recv();
            let response = self.from_server.recv();

            select! {
                request = request => self.handle_request(request).await,
                Some(S2C::Response(response)) = response => self.handle_response(response).await,
                else => break,
            }
        }
    }

    async fn handle_request(&mut self, request: Request) {
        self.to_server
            .send(C2S::Request(self.id, request))
            .await
            .expect("to_server closed");
    }

    async fn handle_response(&mut self, response: Response) {
        self.stream.send(&response).await;
    }
}
