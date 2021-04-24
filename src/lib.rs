use std::io::Write;
use std::sync::atomic::*;
use std::{convert::TryInto, io::Read, iter::repeat_with};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub const MSG_SIZE: usize = 32;

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
    pub fn handle_socket(
        &self,
        mut socket: std::net::TcpStream,
        cancel_token: &AtomicBool,
        total_msg_counter: &AtomicU64,
    ) {
        let mut buf = [0u8; MSG_SIZE];

        loop {
            match socket.read(&mut buf) {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::Relaxed);

            let mut multiplication_result = [0u8; MSG_SIZE];

            for i in 0..MSG_SIZE {
                multiplication_result[i] = buf[i] ^ self.rand[i];
            }

            socket.write_all(&multiplication_result).unwrap();
            total_msg_counter.fetch_add(1, Ordering::Relaxed);

            match socket.read(&mut buf) {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::Relaxed);

            assert_eq!(buf, self.rand);

            if cancel_token.load(Ordering::SeqCst) == true {
                buf[0] = 255;
                socket.write_all(&[0u8; MSG_SIZE]).unwrap();
                total_msg_counter.fetch_add(1, Ordering::Relaxed);
                socket.flush().unwrap();

                return;
            }

            socket.write_all(&[0u8; MSG_SIZE]).unwrap();
            total_msg_counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[allow(arithmetic_overflow)]
    pub async fn handle_socket_async(
        &self,
        mut socket: TcpStream,
        cancel_token: &AtomicBool,
        total_msg_counter: &AtomicU64,
    ) {
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
            total_msg_counter.fetch_add(1, Ordering::Relaxed);

            let mut multiplication_result = [0u8; MSG_SIZE];

            for i in 0..MSG_SIZE {
                multiplication_result[i] = buf[i] ^ self.rand[i];
            }

            socket.write_all(&multiplication_result).await.unwrap();
            total_msg_counter.fetch_add(1, Ordering::Relaxed);

            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::Relaxed);

            assert_eq!(buf, self.rand);

            if cancel_token.load(Ordering::SeqCst) == true {
                buf[0] = 255;
                socket.write_all(&[0u8; MSG_SIZE]).await.unwrap();
                total_msg_counter.fetch_add(1, Ordering::Relaxed);

                socket.flush().await.unwrap();
                return;
            }

            socket.write_all(&[0u8; MSG_SIZE]).await.unwrap();
            total_msg_counter.fetch_add(1, Ordering::Relaxed);
        }
    }
}
