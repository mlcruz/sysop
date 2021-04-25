use std::io::Write;
use std::sync::atomic::*;
use std::{convert::TryInto, io::Read, iter::repeat_with};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

// 2mb
pub const MSG_SIZE: usize = 4 * 1024;
pub const HEAP_MULT: usize = 512;

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
        let rand_block_idx = fastrand::usize(0..HEAP_MULT - 1);

        loop {
            match socket.read(&mut buf) {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::SeqCst);

            for i in 0..MSG_SIZE {
                buf[i] = buf[i] ^ self.rand[i];
            }

            let mut alloc_1mb: Vec<[u8; MSG_SIZE]> = vec![[0; MSG_SIZE]; HEAP_MULT];

            alloc_1mb[rand_block_idx] = buf;
            socket.write_all(&alloc_1mb[rand_block_idx]).unwrap();
            total_msg_counter.fetch_add(1, Ordering::SeqCst);

            match socket.read(&mut buf) {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::SeqCst);

            assert_eq!(buf, self.rand);

            if cancel_token.load(Ordering::SeqCst) == true {
                buf[0] = 255;
                socket.write_all(&[0u8; MSG_SIZE]).unwrap();
                total_msg_counter.fetch_add(1, Ordering::SeqCst);
                socket.flush().unwrap();

                return;
            }

            socket.write_all(&[0u8; MSG_SIZE]).unwrap();
            total_msg_counter.fetch_add(1, Ordering::SeqCst);
            std::mem::drop(alloc_1mb);
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
        let rand_block_idx = fastrand::usize(0..HEAP_MULT - 1);
        loop {
            match socket.read(&mut buf).await {
                // closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::SeqCst);

            for i in 0..MSG_SIZE {
                buf[i] = buf[i] ^ self.rand[i];
            }

            let mut alloc_1mb: Vec<[u8; MSG_SIZE]> = vec![[0; MSG_SIZE]; HEAP_MULT];
            alloc_1mb[rand_block_idx] = buf;
            socket.write_all(&alloc_1mb[rand_block_idx]).await.unwrap();

            total_msg_counter.fetch_add(1, Ordering::SeqCst);

            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            total_msg_counter.fetch_add(1, Ordering::SeqCst);

            assert_eq!(buf, self.rand);

            if cancel_token.load(Ordering::SeqCst) == true {
                buf[0] = 255;
                socket.write_all(&[0u8; MSG_SIZE]).await.unwrap();
                total_msg_counter.fetch_add(1, Ordering::SeqCst);

                socket.flush().await.unwrap();
                return;
            }

            socket.write_all(&[0u8; MSG_SIZE]).await.unwrap();
            total_msg_counter.fetch_add(1, Ordering::SeqCst);
            std::mem::drop(alloc_1mb);
        }
    }
}
