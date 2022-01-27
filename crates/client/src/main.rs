#![allow(unused)]

mod db;
mod ui;

use rat::prelude::*;
use rat::stdin;
use rat::Channel;
use rat::Message;
use rat::Room;
use rat::User;
use tokio::net::TcpStream;

#[tokio::main]
async fn main2() {
    ui::enter();
    ui::main();
    ui::leave();
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:34254";
    let mut stream = TcpStream::connect(addr).await.unwrap();
    println!("Connected to server {}", addr);

    let name = std::env::args().skip(1).next().unwrap_or("anon".into());

    stream.send(&ConnectUser { name }).await;

    let user = match stream.recv().await {
        ConnectedUser(user) => user,
        _ => unreachable!(),
    };

    println!("You are connected, {}!", user.name);

    /*
    let message = prompt().await;
    dbg!(&message);
    stream.send(&Send(message)).await;

    match stream.recv().await {
        Received(author, message) => {
            dbg!(&(author, message));
        }
        _ => unreachable!(),
    }

    stream.send(&Disconnect).await;
    */
}

async fn prompt() -> String {
    use std::io::prelude::*;
    print!("> ");
    std::io::stdout().flush().unwrap();
    stdin().await
}
