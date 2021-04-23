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
    sleep(Duration::from_secs(1)).await;
    let args = std::env::args().into_iter().collect::<Vec<_>>();

    let connection_count = args[1].parse::<usize>().unwrap();

    let mut curr_connections = 0u64;
    let sleep_time = 100;

    let mut tasks = vec![];

    while CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst) < connection_count {
        curr_connections += 1;
        tasks.push(tokio::spawn(async move {
            let mut stream = TcpStream::connect("127.0.0.1:9999").await.unwrap();
            CONNECTIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let mut buf = [0u8; MSG_SIZE];
            let mut rand = [0u8; MSG_SIZE];

            loop {
                let msg: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(MSG_SIZE).collect();

                stream.write_all(&msg).await.unwrap();
                sleep(Duration::from_millis(sleep_time)).await;

                // get rand (from server) * msg
                stream.read_exact(&mut buf).await.unwrap();
                sleep(Duration::from_millis(sleep_time)).await;

                let multiplication_result = buf.clone();

                for i in 0..MSG_SIZE {
                    rand[i] = msg[i] ^ multiplication_result[i];
                }

                // return rand
                stream.write_all(&rand).await.unwrap();
                sleep(Duration::from_millis(sleep_time)).await;

                stream.read_exact(&mut buf).await.unwrap();
                sleep(Duration::from_millis(sleep_time)).await;

                if buf.first().unwrap() == &255u8 {
                    break;
                }

                assert_eq!(&buf, &[0u8; MSG_SIZE]);
                sleep(Duration::from_millis(sleep_time * 6)).await;
            }
            CONNECTIONS.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }));

        if curr_connections == connection_count as u64
            || CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst) % ((connection_count / 25) + 1)
                == 0
        {
            println!("total connections: {}", curr_connections);
            println!(
                "connected: {}",
                CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst) + 1
            );
        }

        sleep(Duration::from_micros(200)).await;
    }

    futures::future::join_all(tasks).await;
}
