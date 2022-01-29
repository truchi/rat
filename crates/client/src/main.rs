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
    let addr = "127.0.0.1:34254";
    let client = Client::connect(addr.into()).await;

    ui::enter();
    let _ = ui::main(client).await;
    ui::leave();
}

async fn prompt() -> String {
    use std::io::prelude::*;
    print!("> ");
    std::io::stdout().flush().unwrap();
    stdin().await
}
