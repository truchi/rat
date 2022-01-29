#![allow(unused)]

pub mod db;
pub mod tasks;

use db::*;
use tasks::*;

use rat::prelude::*;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[doc(hidden)]
macro_rules! mpsc_aliases {
    ($($Type:ident = $Channel:ident<$Message:ident>)*) => { $(
        #[doc = concat!("Alias of `", stringify!($Channel), "<`[`", stringify!($Message), "`]`>`.")]
        pub type $Type = mpsc::$Channel<$Message>;
    )* };
}

mpsc_aliases!(
    FromServer = Receiver<ServerToClient>
    FromClient = Receiver<ClientToServer>
    ToClient   = Sender<ServerToClient>
    ToServer   = Sender<ClientToServer>
);

/// Alias of [`ClientToServer`].
pub type C2S = ClientToServer;

/// Alias of [`ServerToClient`].
pub type S2C = ServerToClient;

/// [`ServerTask`] to [`ClientTask`] messages.
#[derive(Debug)]
pub enum ServerToClient {
    Accepted(ClientId),
    Response(Response),
}

/// [`ClientTask`] to [`ServerTask`] messages.
#[derive(Debug)]
pub enum ClientToServer {
    Accept(ToClient),
    Request(ClientId, Request),
}

/// Runs the server.
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:34254";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server started on {addr}");

    let (to_server, from_client) = mpsc::channel(32);

    let server = tokio::spawn(async move {
        println!("ServerTask spawned!");
        ServerTask::new(from_client).run().await;
        println!("ServerTask done!");
    });

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let to_server = to_server.clone();

        tokio::spawn(async move {
            println!("ClientTask spawned!");

            if let Ok(mut task) = ClientTask::new(stream, to_server).await {
                task.run().await;
            }

            println!("ClientTask done!");
        });
    }
}
