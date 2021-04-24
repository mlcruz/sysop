use std::sync::atomic::*;
use std::time::Duration;
use sysop::Actor;

static CANCELATION: AtomicBool = AtomicBool::new(false);
static TOTAL_MSGS: AtomicU64 = AtomicU64::new(0);

fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();

    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

    let timeout = args[1].parse::<u128>().unwrap();

    let timeout_thread = std::thread::spawn(move || {
        let timer = std::time::Instant::now();

        loop {
            if timer.elapsed().as_millis() > timeout * 1000 {
                CANCELATION.swap(true, Ordering::Relaxed);
                break;
            }

            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    std::thread::spawn(move || loop {
        let (socket, _) = listener.accept().unwrap();

        std::thread::spawn(move || {
            let actor = Actor::new();

            actor.handle_socket(socket, &CANCELATION, &TOTAL_MSGS);
        });
    });

    timeout_thread.join().unwrap();
    println!("Total msgs: {}", TOTAL_MSGS.load(Ordering::Relaxed));
}
