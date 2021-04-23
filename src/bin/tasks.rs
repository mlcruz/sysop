use std::time::Duration;
use sysop::Actor;
use tokio::net::TcpListener;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();
    let listener = TcpListener::bind("127.0.0.1:9999").await.unwrap();
    static CANCELATION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let timeout = args[1].parse::<u128>().unwrap();

    tokio::spawn(async move {
        let timer = std::time::Instant::now();

        loop {
            if timer.elapsed().as_millis() > timeout * 1000 {
                CANCELATION.swap(true, std::sync::atomic::Ordering::SeqCst);
                break;
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let actor = Actor::new();

                actor.handle_socket_async(socket, &CANCELATION).await;
            });
        }
    });

    loop {
        if CANCELATION.load(std::sync::atomic::Ordering::SeqCst) {
            return ();
        }

        sleep(Duration::from_millis(100)).await;
    }
}
