#![allow(unused)]

pub mod ring;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// A user.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

/// A room.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
}

/// A message.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Message {
    pub body: String,
}

/// An event.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Event {
    Enter {
        channel: Channel,
        user:    User,
    },
    Leave {
        channel: Channel,
        user:    User,
    },
    Post {
        channel: Channel,
        user:    User,
        message: Message,
    },
}

/// A channel.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Channel {
    World,
    Room { room: Room },
    Private { user: User },
}

pub async fn stdin() -> String {
    use tokio::io::stdin;
    let mut buffer = [0; CAP];
    let n = stdin().read(&mut buffer).await.unwrap();
    String::from_utf8_lossy(&buffer[..n]).into_owned()
}

pub const CAP: usize = 8 * 1024;

#[async_trait]
pub trait StreamExt {
    async fn send<T: Sync + Serialize>(&mut self, value: &T);
    async fn recv<T: DeserializeOwned>(&mut self) -> T;
}

#[async_trait]
impl StreamExt for TcpStream {
    async fn send<T: Sync + Serialize>(&mut self, value: &T) {
        self.write(ron::ser::to_string(value).unwrap().as_bytes())
            .await
            .unwrap();
    }

    async fn recv<T: DeserializeOwned>(&mut self) -> T {
        let mut buffer = [0; CAP];
        let n = self.read(&mut buffer).await.unwrap();
        ron::de::from_bytes(&buffer[..n]).unwrap()
    }
}

pub use ClientRequest::*;
#[derive(Deserialize, Serialize, Debug)]
pub enum ClientRequest {
    ConnectUser(User),
    DisconnectUser(User),
    Event(Event),
}

pub use ServerResponse::*;
#[derive(Deserialize, Serialize, Debug)]
pub enum ServerResponse {
    ConnectedUser(User),
    DisconnectedUser(User),
    Evented(Event),
}
