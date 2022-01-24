use chat::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let name = std::env::args().skip(1).next().unwrap_or("anon".into());

    let mut stream = TcpStream::connect("127.0.0.1:34254").await.unwrap();
    stream.send(&Connect(name)).await;

    let name = match stream.recv().await {
        Connected(name) => name,
        _ => unreachable!(),
    };

    println!("You are connected, {name}!");

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
}

async fn prompt() -> String {
    use std::io::prelude::*;
    print!("> ");
    std::io::stdout().flush().unwrap();
    stdin().await
}
