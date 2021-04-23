use std::{iter::repeat_with, time::Duration};
use tokio::time::sleep;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub const MSG_SIZE: usize = 8;

#[repr(transparent)]
pub struct Actor {
    rand: [u8; 8],
}

impl Actor {
    pub fn new() -> Actor {
        Self {
            rand: [
                fastrand::u8(..),
                fastrand::u8(..),
                fastrand::u8(..),
                fastrand::u8(..),
                fastrand::u8(..),
                fastrand::u8(..),
                fastrand::u8(..),
                fastrand::u8(..),
            ],
        }
    }

    pub async fn handle_socket_async(&self, mut socket: TcpStream) {
        let mut buf = [0u8; MSG_SIZE];

        loop {
            match socket.read_exact(&mut buf).await {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };

            sleep(Duration::from_millis(10)).await;

            if let Err(e) = socket.write_all(&self.rand).await {
                panic!("{}", e);
            }

            let mut multiplication_result = [0u8; 8];

            for i in 0..MSG_SIZE {
                multiplication_result[i] = self.rand[i] * buf[i];
            }

            sleep(Duration::from_millis(10)).await;

            if let Err(e) = socket.write_all(&multiplication_result).await {
                panic!("{}", e);
            }

            match socket.read_exact(&mut buf).await {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };

            assert_eq!(buf, multiplication_result);

            if let Err(e) = socket.write_all(&[0u8; 8]).await {
                panic!("{}", e);
            }
        }
    }
}
