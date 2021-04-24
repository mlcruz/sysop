use std::sync::atomic::*;
use std::time::Duration;
use sysop::Actor;
use tokio::net::TcpListener;
use tokio::time::sleep;

static TOTAL_MSGS: AtomicU64 = AtomicU64::new(0);
static CANCELATION: AtomicBool = AtomicBool::new(false);
static TOTAL_CONN: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();
    let listener = TcpListener::bind("127.0.0.1:9999").await.unwrap();
    let timeout = args[1].parse::<u128>().unwrap();
    fastrand::seed(7);

    let timeout_task = tokio::spawn(async move {
        let timer = std::time::Instant::now();

        loop {
            if timer.elapsed().as_millis() > timeout * 1000 {
                CANCELATION.swap(true, Ordering::SeqCst);
                break;
            }

            sleep(Duration::from_millis(2000)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let actor = Actor::new();

                TOTAL_CONN.fetch_add(1, Ordering::SeqCst);
                actor
                    .handle_socket_async(socket, &CANCELATION, &TOTAL_MSGS)
                    .await;
            });
        }
    });

    timeout_task.await.unwrap();
    println!("TaskTotalMsgs,{}", TOTAL_MSGS.load(Ordering::SeqCst));
    println!("TaskTotalConn,{}", TOTAL_CONN.load(Ordering::SeqCst));
}
