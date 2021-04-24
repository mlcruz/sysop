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
    fastrand::seed(7);

    let connection_count = args[1].parse::<usize>().unwrap();

    let msg_interval = 100;

    let mut tasks = vec![];

    while CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst) < connection_count {
        tasks.push(tokio::spawn(async move {
            let mut stream = TcpStream::connect("127.0.0.1:9999").await.unwrap();
            CONNECTIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let mut buf = [0u8; MSG_SIZE];
            let mut rand = [0u8; MSG_SIZE];

            let curr_conn = CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst);
            if curr_conn == connection_count || curr_conn % ((connection_count / 25) + 1) == 0 {
                println!("connected: {}", curr_conn);
            }

            loop {
                let msg: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(MSG_SIZE).collect();

                stream.write_all(&msg).await.unwrap();
                sleep(Duration::from_millis(msg_interval)).await;

                // get rand (from server) * msg
                stream.read_exact(&mut buf).await.unwrap();
                sleep(Duration::from_millis(msg_interval)).await;

                let multiplication_result = buf.clone();

                for i in 0..MSG_SIZE {
                    rand[i] = msg[i] ^ multiplication_result[i];
                }

                // return rand
                stream.write_all(&rand).await.unwrap();
                sleep(Duration::from_millis(msg_interval)).await;

                stream.read_exact(&mut buf).await.unwrap();
                sleep(Duration::from_millis(msg_interval)).await;

                if buf.first().unwrap() == &255u8 {
                    break;
                }

                assert_eq!(&buf, &[0u8; MSG_SIZE]);
                sleep(Duration::from_millis(msg_interval)).await;
            }

            CONNECTIONS.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }));

        sleep(Duration::from_micros(100)).await;
    }

    futures::future::join_all(tasks).await;
}
