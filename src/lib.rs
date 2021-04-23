use std::{convert::TryInto, iter::repeat_with, time::Duration};
use tokio::time::sleep;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub const MSG_SIZE: usize = 128;

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
    pub async fn handle_socket_async(&self, mut socket: TcpStream) {
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

            // println!("{:?}", &buf);

            sleep(Duration::from_millis(10)).await;

            if let Err(e) = socket.write_all(&self.rand).await {
                panic!("{}", e);
            }

            // println!("{:?}", self.rand);
            let mut multiplication_result = [0u8; MSG_SIZE];

            for i in 0..MSG_SIZE {
                multiplication_result[i] = self.rand[i] * buf[i];
            }

            sleep(Duration::from_millis(10)).await;

            if let Err(e) = socket.write_all(&multiplication_result).await {
                panic!("{}", e);
            }

            match socket.read(&mut buf).await {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };

            // println!("{:?}", multiplication_result);

            assert_eq!(buf, multiplication_result);

            if let Err(e) = socket.write_all(&[0u8; MSG_SIZE]).await {
                panic!("{}", e);
            }
        }
    }
}
