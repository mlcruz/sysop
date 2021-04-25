use std::{iter::repeat_with, sync::atomic::AtomicUsize, time::Duration};

use std::sync::atomic::*;
use sysop::MSG_SIZE;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::sleep,
};

static CONNECTIONS: AtomicUsize = AtomicUsize::new(0);
static CANCELATION: AtomicBool = AtomicBool::new(false);
#[tokio::main]
#[allow(arithmetic_overflow)]
async fn main() {
    sleep(Duration::from_secs(1)).await;
    let args = std::env::args().into_iter().collect::<Vec<_>>();
    fastrand::seed(7);

    let connection_count = args[1].parse::<usize>().unwrap();

    let msg_interval = 500;

    let mut tasks = vec![];

    while CONNECTIONS.load(Ordering::SeqCst) < connection_count {
        if CANCELATION.load(Ordering::SeqCst) == true {
            break;
        }

        tasks.push(tokio::spawn(async move {
            let stream = TcpStream::connect("127.0.0.1:9999").await;
            if stream.is_err() {
                CANCELATION.swap(true, Ordering::SeqCst);
                return;
            };
            let mut stream = stream.unwrap();
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

                for i in 0..MSG_SIZE {
                    rand[i] = msg[i] ^ buf[i];
                }

                // return rand
                stream.write_all(&rand).await.unwrap();
                sleep(Duration::from_millis(msg_interval)).await;

                stream.read_exact(&mut buf).await.unwrap();
                sleep(Duration::from_millis(msg_interval)).await;

                if buf.first().unwrap() == &255u8 {
                    CANCELATION.swap(true, Ordering::SeqCst);
                    break;
                }

                assert_eq!(&buf, &[0u8; MSG_SIZE]);
            }
        }));

        sleep(Duration::from_millis(5)).await;
    }

    futures::future::join_all(tasks).await;
}
