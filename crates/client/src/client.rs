use super::*;
use tokio::net::ToSocketAddrs;

pub struct Client {
    id:     ClientId,
    stream: TcpStream,
    db:     Option<Db>,
}

impl Client {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Self {
        let mut stream = TcpStream::connect(addr).await.unwrap();

        let id = match stream.recv().await {
            Response::Accepted(client) => client.id,
            _ => unreachable!(), // TODO
        };

        Self {
            id,
            stream,
            db: None,
        }
    }

    pub async fn connect_user(&mut self, name: String) {
        self.stream.send(&Request::Connect(name)).await;

        let user = match self.stream.recv().await {
            Response::Connected(user) => user,
            _ => unreachable!(), // TODO
        };

        self.db = Some(Db::new(user));

        // dbg!(stream.recv::<Response>().await);
    }
}
