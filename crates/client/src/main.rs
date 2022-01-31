#![allow(unused)]

mod client;
mod db;
mod ui;

use client::*;
use db::*;
use rat::prelude::*;
use rat::stdin;
use rat::Message;
use rat::Room;
use rat::User;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let name = if let Some(name) = std::env::args().skip(1).next() {
        name
    } else {
        println!("Your name as first argument!");
        return;
    };

    let addr = "127.0.0.1:34254";
    let client = Client::connect(addr.into()).await;
    let mut connection = client.connect_user(name).await;
    connection.enter_world().await;

    ui::enter();
    ui::main(connection).await;
    ui::leave();
}

async fn prompt() -> String {
    use std::io::prelude::*;
    print!("> ");
    std::io::stdout().flush().unwrap();
    stdin().await
}
