use super::*;
use ron::de;
use ron::ser;
use std::io;
use std::io::ErrorKind;

/// Capacity for buffers.
pub const CAP: usize = 10 * 1024;

const SER_ERROR: &str = "Serialization error";
const DE_ERROR: &str = "Deserialization error";

/// Serde [`send`](StreamExt::send) and [`recv`](StreamExt::recv)
/// extensions for `TcpStream`s.
#[async_trait]
pub trait StreamExt {
    async fn send<T: Sync + Serialize>(&mut self, value: &T) -> io::Result<()>;
    async fn recv<T: DeserializeOwned>(&mut self) -> io::Result<T>;
}

#[async_trait]
impl StreamExt for TcpStream {
    async fn send<T: Sync + Serialize>(&mut self, value: &T) -> io::Result<()> {
        let string = ser::to_string_pretty(value, Default::default()).expect(SER_ERROR);
        self.write_all(string.as_bytes()).await?;
        println!("Sent: {}", string);
        Ok(())
    }

    async fn recv<T: DeserializeOwned>(&mut self) -> io::Result<T> {
        const CHUNK: usize = 1024;

        let mut buffer = Vec::new();
        let mut n = 0;

        loop {
            self.readable().await.unwrap();
            debug_assert!(n + CHUNK >= buffer.len());
            buffer.resize(n + CHUNK, 0);

            match self.try_read(&mut buffer[n..]) {
                Ok(0) if n == 0 => return Err(io::Error::new(ErrorKind::Other, "Cannot recv()")),
                Ok(m) => n += m,
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }

        println!("Received: {}", std::str::from_utf8(&buffer[..n]).unwrap());
        Ok(de::from_bytes(&buffer[..n]).expect(DE_ERROR))
    }
}
