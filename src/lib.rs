use serde::{Deserialize, Serialize};
use std::io::Write;
use std::sync::atomic::*;
use std::{convert::TryInto, io::Read, iter::repeat_with};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

// 512kb
pub const MSG_SIZE: usize = 512;
pub const HEAP_MULT: usize = 1024;

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
        let rand_block_idx = fastrand::usize(0..HEAP_MULT);
        let mut alloc_1mb: Vec<[u8; MSG_SIZE]> = vec![[0; MSG_SIZE]; HEAP_MULT];

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
        let rand_block_idx = fastrand::usize(0..HEAP_MULT);
        let mut alloc_1mb: Vec<[u8; MSG_SIZE]> = vec![[0; MSG_SIZE]; HEAP_MULT];

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
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MergedBenchResult {
    pub msg_size: String,
    pub connections: u128,
    pub kind: String,
    pub counts: u128,
    pub cycles: u128,
    pub instructions: u128,
    pub faults: u128,
    pub minor_faults: u128,
    pub major_faults: u128,
    pub bus_cycles: u128,
    pub d_tlb_loads: u128,
    pub d_tlb_load_misses: u128,
    pub llc_loads: u128,
    pub llc_load_misses: u128,
    pub l1_cache_loads: u128,
    pub l1_cache_load_misses: u128,
    pub l1_cache_stores: u128,
    pub llc_stores: u128,
    pub cache_references: u128,
    pub cache_misses: u128,
    pub branch_instructions: u128,
    pub d_tlb_stores: u128,
    pub d_tlb_store_misses: u128,
    pub branch_misses: u128,
    pub context_switches: u128,
}
