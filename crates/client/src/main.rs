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
    ui::main(client).await;
    ui::leave();
}

#[tokio::main]
async fn main2() {
    let addr = "127.0.0.1:34254";
    let mut stream = TcpStream::connect(addr).await.unwrap();

    let client = match stream.recv().await {
        Response::Accepted(client) => client,
        _ => unreachable!(),
    };

    println!("Connected to server {}", addr);

    let name = std::env::args().skip(1).next().unwrap_or("anon".into());

    stream.send(&Request::Connect(name)).await;

    let user = match stream.recv().await {
        Response::Connected(user) => user,
        x => unreachable!("{:?}", x),
    };

    println!("You are connected, {}!", user.name);
    dbg!(stream.recv::<Response>().await);

    {
        stream.send(&Request::GetUser(user.name)).await;
        let recv = stream.recv::<Response>().await;
        dbg!(&recv);
        stream.send(&Request::GetUser("lkjqhsdlkqjhd".into())).await;
        let recv = stream.recv::<Response>().await;
        dbg!(&recv);
        stream.send(&Request::CreateRoom("my_room".into())).await;
        let recv = stream.recv::<Response>().await;
        dbg!(&recv);
        stream.send(&Request::GetRoom("my_room".into())).await;
        let room = match stream.recv::<Response>().await {
            Response::Room(room) => room.unwrap(),
            x => unreachable!("{:?}", x),
        };
        dbg!(&room);
        stream
            .send(&Request::Event(user.id.enter_room(room.id)))
            .await;
        let recv = stream.recv::<Response>().await;
        dbg!(&recv);
    }

    loop {
        let recv = stream.recv::<Response>().await;
        dbg!(&recv);
    }
}

async fn prompt() -> String {
    use std::io::prelude::*;
    print!("> ");
    std::io::stdout().flush().unwrap();
    stdin().await
}
