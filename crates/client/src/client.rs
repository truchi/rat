use super::*;
use tokio::net::ToSocketAddrs;

pub struct Client {
    id:     ClientId,
    addr:   String,
    stream: TcpStream,
    db:     Option<Db>,
}

impl Client {
    pub async fn connect(addr: String) -> Self {
        let mut stream = TcpStream::connect(&addr).await.unwrap();

        let id = match stream.recv().await.expect("Server error") {
            Response::Accepted(client) => client.id,
            _ => unreachable!(), // TODO
        };

        Self {
            id,
            addr,
            stream,
            db: None,
        }
    }

    pub async fn connect_user(&mut self, name: String) {
        self.stream.send(&Request::Connect(name)).await;

        let user = match self.stream.recv().await.expect("Server error") {
            Response::Connected(user) => user,
            _ => unreachable!(), // TODO
        };

        self.db = Some(Db::new(user));

        // dbg!(stream.recv::<Response>().await);
    }
}
