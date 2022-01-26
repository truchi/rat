#![allow(unused)]

mod client;
mod manager;
mod world;

use client::*;
use manager::*;
use world::*;

use rat::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type FromManager = mpsc::Receiver<ManagerToClient>;
pub type FromClient = mpsc::Receiver<ClientToManager>;
pub type ToClient = mpsc::Sender<ManagerToClient>;
pub type ToManager = mpsc::Sender<ClientToManager>;

pub use ManagerToClient::*;
#[derive(Debug)]
pub enum ManagerToClient {
    Accepted(Uuid),
    Response(ServerResponse),
}

pub use ClientToManager::*;
#[derive(Debug)]
pub enum ClientToManager {
    Accept(ToClient),
    Request(Uuid, ClientRequest),
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:34254";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server started on {addr}");

    let (to_manager, from_client) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        Manager::new(from_client).run().await;
    });

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let to_manager = to_manager.clone();

        tokio::spawn(async move {
            if let Ok(mut client) = Client::new(stream, to_manager).await {
                client.run().await;
            }
        });
    }
}

/*
#[derive(Debug)]
struct Client {
    pub name:      Option<String>,
    pub to_client: ToClient,
}

impl Client {
    pub fn new(to_client: ToClient) -> Self {
        Self {
            name: None,
            to_client,
        }
    }
}

#[derive(Default, Debug)]
struct Clients {
    id:      ClientId,
    clients: HashMap<ClientId, Client>,
}

impl Clients {
    pub async fn init(&mut self, to_client: ToClient) -> ClientId {
        let id = self.id;
        self.id += 1;
        self.clients.insert(id, Client::new(to_client));
        self.get_mut(id).to_client.inited(id).await;

        id
    }

    pub fn get(&self, id: ClientId) -> &Client {
        self.clients.get(&id).unwrap()
    }

    pub fn get_mut(&mut self, id: ClientId) -> &mut Client {
        self.clients.get_mut(&id).unwrap()
    }

    pub async fn connect(&mut self, id: ClientId, name: String) {
        // TODO check name
        let client = self.get_mut(id);
        client.name = Some(name.clone());
        client.to_client.response(Connected(name)).await;
    }

    pub async fn send(&mut self, id: ClientId, message: String) {
        let author = self.get(id).name.clone().unwrap();

        for (_, Client { name, to_client }) in &mut self.clients {
            if name.is_some() {
                to_client
                    .response(Received(author.clone(), message.clone()))
                    .await;
            }
        }
    }
}

#[derive(Debug)]
struct ToClient(pub mpsc::Sender<ManagerToClient>);

impl ToClient {
    pub async fn inited(&mut self, id: ClientId) {
        self.0.send(ManagerToClient::Inited(id)).await;
    }

    pub async fn response(&mut self, response: ServerResponse) {
        self.0.send(ManagerToClient::Response(response)).await;
    }
}

impl From<mpsc::Sender<ManagerToClient>> for ToClient {
    fn from(to_client: mpsc::Sender<ManagerToClient>) -> Self {
        Self(to_client)
    }
}

#[derive(Debug)]
struct ToManager(pub mpsc::Sender<ClientToManager>);

impl ToManager {
    pub async fn init(&mut self, to_client: impl Into<ToClient>) {
        self.0.send(ClientToManager::Init(to_client.into())).await;
    }

    pub async fn request(&mut self, id: ClientId, request: ClientRequest) {
        self.0.send(ClientToManager::Request(id, request)).await;
    }
}

impl From<mpsc::Sender<ClientToManager>> for ToManager {
    fn from(to_manager: mpsc::Sender<ClientToManager>) -> Self {
        Self(to_manager)
    }
}

struct ManagerTask {
    from_client: FromClient,
    clients:     Clients,
}

impl ManagerTask {
    pub fn new(from_client: FromClient) -> Self {
        Self {
            from_client,
            clients: Clients::default(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(command) = self.from_client.recv().await {
            match command {
                ClientToManager::Init(to_client) => {
                    self.clients.init(to_client).await;
                }
                ClientToManager::Request(id, Connect(name)) => {
                    self.clients.connect(id, name).await;
                }
                ClientToManager::Request(id, Send(message)) => {
                    println!("message {message}");
                    self.clients.send(id, message).await;
                }
                _ => {}
            }
        }
    }
}
*/
