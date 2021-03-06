use super::*;
use tokio::net::ToSocketAddrs;

#[derive(Debug)]
pub struct Client {
    id:     ClientId,
    addr:   String,
    stream: TcpStream,
}

impl Client {
    pub async fn connect(addr: String) -> Self {
        let mut stream = TcpStream::connect(&addr).await.unwrap();

        let id = match stream.recv().await.expect("Server error") {
            Response::Accepted(client) => client.id,
            _ => unreachable!(), // TODO
        };

        Self { id, addr, stream }
    }

    pub async fn connect_user(mut self, name: String) -> Connection {
        self.stream.send(&Request::Connect(name)).await;

        let user = match self.stream.recv().await.expect("Server error") {
            Response::Connected(user) => user,
            _ => unreachable!(), // TODO
        };

        Connection::new(self, user)
    }
}

#[derive(Debug)]
pub struct Connection {
    client: Client,
    db:     Db,
}

impl Connection {
    pub fn new(client: Client, user: User) -> Self {
        Self {
            client,
            db: Db::new(user),
        }
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn user(&self) -> &User {
        self.db.user()
    }

    pub async fn next(&mut self) {
        let response = self.recv().await;
        match response {
            Response::Event(event) => self.db.push(event),
            _ => unreachable!(), // TODO
        }
    }

    pub async fn enter_world(&mut self) {
        self.send(Request::Event(self.user().id.enter_world()))
            .await;
    }

    pub async fn leave_world(&mut self) {
        self.send(Request::Event(self.user().id.leave_world()))
            .await;
    }

    pub async fn post_world(&mut self, message: String) {
        self.send(Request::Event(
            self.user().id.post_world(Message { body: message }),
        ))
        .await;
    }

    async fn send(&mut self, request: Request) {
        self.client
            .stream
            .send(&request)
            .await
            .expect("Server error");
    }

    async fn recv(&mut self) -> Response {
        self.client.stream.recv().await.expect("Server error")
    }
}
