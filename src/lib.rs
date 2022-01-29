#![allow(unused)]

pub mod ring;

mod event;
mod stream_ext;

pub use stream_ext::*;

pub mod prelude {
    pub use super::Channel;
    pub use super::ClientId;
    pub use super::Event;
    pub use super::EventType;
    pub use super::Request;
    pub use super::Response;
    pub use super::RoomId;
    pub use super::StreamExt;
    pub use super::UserId;
}

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use uuid::Uuid;

macro_rules! ids {
    ($($(#[$doc:meta])* $Id:ident)*) => { $(
        $(#[$doc])*
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
        pub struct $Id(Uuid);

        impl $Id {
            #[doc = concat!("Creates a new `", stringify!($Id), "`.")]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
        }
    )* };
}

ids!(
    /// A [`Client`] id.
    ClientId
    /// A [`User`] id.
    UserId
    /// A [`Room`] id.
    RoomId
);

/// A client.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
}

/// A user.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct User {
    pub id:   UserId,
    pub name: String,
}

/// A room.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Room {
    pub id:   RoomId,
    pub name: String,
}

/// A message.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Message {
    pub body: String,
}

/// An event.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Event<U, R> {
    pub channel:    Channel<R>,
    pub user:       U,
    pub event_type: EventType,
}

impl<U, R> Event<U, R> {
    pub fn new(channel: Channel<R>, user: U, event_type: EventType) -> Self {
        Self {
            channel,
            user,
            event_type,
        }
    }
}

/// An [`Event`] type.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum EventType {
    Enter,
    Leave,
    Post { message: Message },
}

/// A channel.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Channel<R> {
    World,
    Room(R),
}

/// A [`Client`] request to the server.
#[derive(Deserialize, Serialize, Debug)]
pub enum Request {
    GetUser(String),
    GetRoom(String),
    Connect(String),
    CreateRoom(String),
    Event(Event<UserId, RoomId>),
    Disconnect,
    Shutdown,
}

/// A server response to the [`Client`].
#[derive(Deserialize, Serialize, Debug)]
pub enum Response {
    Accepted(Client),
    User(Option<User>),
    Room(Option<Room>),
    Connected(User),
    CreatedRoom(Room),
    Event(Event<User, Room>),
    Disconnected,
    Error,
}

pub async fn stdin() -> String {
    use tokio::io::stdin;
    let mut buffer = [0; CAP];
    let n = stdin().read(&mut buffer).await.unwrap();
    String::from_utf8_lossy(&buffer[..n]).into_owned()
}
