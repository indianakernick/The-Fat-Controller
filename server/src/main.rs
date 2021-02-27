mod socket;

use tokio::{net::TcpListener, sync::mpsc};

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut tfc_ctx = tfc::Context::new().unwrap();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<tfc::Command>();
    let socket_ctx = socket::SocketContext::new(ch_tx);

    // Is spawning this task necessary?
    // If we only want one connection at a time then can't we do everything from
    // the main thread?
    tokio::spawn(async move {
        let listener = match TcpListener::bind("0.0.0.0:80").await {
            Ok(l) => l,
            Err(e) => {
                println!("Bind: {}", e);
                return;
            },
        };

        println!("Listening on port 80");

        loop {
            let (stream, addr) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    println!("Accept: {}", e);
                    continue;
                },
            };

            // The handle_stream function will async-block for the duration of
            // the connection which means that only one client can connect to
            // the server at a time. This is by design. We don't want multiple
            // clients.
            println!("Connected to {}", addr);
            match socket_ctx.handle_stream(stream).await {
                Ok(()) => println!("Disconnected"),
                Err(e) => println!("Receive: {}", e),
            }
        }
    });

    while let Some(command) = ch_rx.recv().await {
        if let Err(e) = command.execute(&mut tfc_ctx) {
            println!("Execute: {}", e);
        }
    }
}
