use std::{convert::TryInto, iter::repeat_with, sync::atomic::AtomicBool};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub const MSG_SIZE: usize = 8;

#[repr(transparent)]
pub struct Actor {
    rand: [u8; MSG_SIZE],
}

impl Actor {
    pub fn new() -> Actor {
        let rand: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(MSG_SIZE).collect();
        Self {
            rand: rand.try_into().unwrap(),
        }
    }
    #[allow(arithmetic_overflow)]
    pub async fn handle_socket_async(&self, mut socket: TcpStream, cancel_token: &AtomicBool) {
        let mut buf = [0u8; MSG_SIZE];

        loop {
            match socket.read(&mut buf).await {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };

            let mut multiplication_result = [0u8; MSG_SIZE];

            for i in 0..MSG_SIZE {
                multiplication_result[i] = buf[i] ^ self.rand[i];
            }

            socket.write_all(&multiplication_result).await.unwrap();

            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };

            assert_eq!(buf, self.rand);

            if cancel_token.load(std::sync::atomic::Ordering::SeqCst) == true {
                buf[0] = 255;
            }

            socket.write_all(&[0u8; MSG_SIZE]).await.unwrap();
        }
    }
}
