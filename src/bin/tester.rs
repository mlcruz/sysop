use std::{iter::repeat_with, sync::atomic::AtomicUsize, time::Duration};

use sysop::MSG_SIZE;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::sleep,
};

static CONNECTIONS: AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();

    let connection_count = args[1].parse::<usize>().unwrap();

    let mut curr_connections = 0u64;

    let mut tasks = vec![];

    while CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst) < connection_count {
        if curr_connections % 10 == 0
            || CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst) % 5 == 0
        {
            println!("total connections: {}", curr_connections);
            println!(
                "connected: {}",
                CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst)
            );
        }

        curr_connections += 1;
        tasks.push(tokio::spawn(async move {
            let mut stream = TcpStream::connect("127.0.0.1:9999").await.unwrap();

            CONNECTIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            let msg: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(MSG_SIZE).collect();
            let sleep_time = fastrand::u8(1..200) as u64 * (fastrand::u8(1..100) / 2) as u64;

            // println!("{:?}", &msg);
            stream.write_all(&msg).await.unwrap();
            sleep(Duration::from_millis(sleep_time / 2)).await;

            let mut buf = [0u8; MSG_SIZE];

            stream.read_exact(&mut buf).await.unwrap();

            let seed = buf.clone();
            // println!("{:?}", &seed);

            sleep(Duration::from_millis(sleep_time / 3)).await;

            let mut multiplication_result = [0u8; MSG_SIZE];

            for i in 0..MSG_SIZE {
                multiplication_result[i] = seed[i] * msg[i];
            }

            stream.read_exact(&mut buf).await.unwrap();
            let result = buf.clone();
            // println!("{:?}", multiplication_result);

            assert_eq!(result, multiplication_result);

            sleep(Duration::from_millis(200)).await;
            stream.write_all(&result).await.unwrap();

            sleep(Duration::from_millis(sleep_time / 5)).await;
            stream.read_exact(&mut buf).await.unwrap();

            assert_eq!(&buf, &[0u8; MSG_SIZE]);
            // println!("{} done", tcp_id);
            CONNECTIONS.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }));

        sleep(Duration::from_micros(200)).await;
    }

    futures::future::join_all(tasks).await;
}
