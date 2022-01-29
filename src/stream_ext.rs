use super::*;

/// Capacity for buffers.
pub const CAP: usize = 10 * 1024;

/// Serde [`send`](StreamExt::send) and [`recv`](StreamExt::recv)
/// extensions for `TcpStream`s.
#[async_trait]
pub trait StreamExt {
    async fn send<T: Sync + Serialize + std::fmt::Debug>(&mut self, value: &T);
    async fn recv<T: DeserializeOwned>(&mut self) -> T;
}

#[async_trait]
impl StreamExt for TcpStream {
    async fn send<T: Sync + Serialize + std::fmt::Debug>(&mut self, value: &T) {
        self.write_all(ron::ser::to_string(value).unwrap().as_bytes())
            .await
            .unwrap();
        println!("Sent {:?}", value);
    }

    async fn recv<T: DeserializeOwned>(&mut self) -> T {
        let mut buffer = [0; CAP];
        let n = self.read(&mut buffer).await.unwrap();
        ron::de::from_bytes(&buffer[..n]).unwrap()
    }

    /*
    async fn recv<T: DeserializeOwned>(&mut self) -> T {
        const CHUNK: usize = 1024;

        let mut buffer = Vec::new();
        let mut n = 0;

        loop {
            dbg!(n);
            self.readable().await.unwrap();
            buffer.resize(n + CHUNK, 0);
            println!("readable {}", &mut buffer[n..].len());

            match self.try_read(&mut buffer[n..]) {
                Ok(0) => {
                    break;
                }
                Ok(m) => {
                    n += m;
                }
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    dbg!(e);
                    panic!();
                    // return Err(e.into());
                }
            }
        }

        dbg!(utf8(&buffer[..n]));
        ron::de::from_bytes(&buffer[..n]).unwrap()
    }
    */

    /*
    async fn recv2<T: DeserializeOwned>(&mut self) -> T {
        let mut buffer = Vec::new();
        loop {
            let n = self.read_buf(&mut buffer).await.unwrap();
            dbg!(n);
            match std::str::from_utf8(&buffer) {
                Ok(str) => {
                    dbg!(str);
                }
                Err(err) => {
                    let n = err.valid_up_to();
                    dbg!(&buffer[..n]);
                }
            }
            // if n == 0 {
            // break;
            // }
        }
        ron::de::from_bytes(&buffer[..]).unwrap()
    }
    */
}

fn utf8(str: &[u8]) -> &str {
    match std::str::from_utf8(str) {
        Ok(str) => str,
        Err(err) => {
            let n = err.valid_up_to();
            dbg!("valid up to", n);
            unsafe { std::str::from_utf8_unchecked(&str[..n]) }
        }
    }
}
