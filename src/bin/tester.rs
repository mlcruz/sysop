use std::{iter::repeat_with, sync::Arc, time::Duration};

use sysop::{Actor, MSG_SIZE};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{unix::SocketAddr, TcpListener, TcpStream},
    sync::Mutex,
    time::sleep,
};

#[tokio::main]
async fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();

    let connection_count = args[0].parse::<u64>().unwrap();

    let mut curr_connections = 0u64;

    while curr_connections < connection_count {
        if connection_count % 10 == 0 {
            println!("{}", curr_connections);
        }

        curr_connections += 1;
        tokio::spawn(async move {
            let mut stream = TcpStream::connect("127.0.0.1:9999").await.unwrap();
            let msg: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(MSG_SIZE).collect();

            stream.write_all(&msg).await.unwrap();
            sleep(Duration::from_millis(20)).await;

            let mut buf = [0u8; MSG_SIZE];

            stream.read_exact(&mut buf).await.unwrap();

            let seed = buf.clone();
            sleep(Duration::from_millis(20)).await;

            stream.read_exact(&mut buf).await.unwrap();
            let result = buf.clone();

            stream.write_all(&result).await.unwrap();

            stream.read_exact(&mut buf).await.unwrap();
        });

        sleep(Duration::from_millis(50)).await;
    }
}
