use sysop::Actor;
use tokio::net::TcpListener;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let actor = Actor::new();

            actor.handle_socket_async(socket).await;
        });
    }
}
