mod socket;

use tokio::net::TcpListener;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let tfc_ctx = match tfc::Context::new() {
        Ok(c) => c,
        Err(e) => {
            println!("Initialize: {}", e);
            return;
        }
    };
    let mut socket_ctx = socket::SocketContext::new(tfc_ctx);

    // If we're doing everything on the main thread then do we even need tokio
    // at all? Why not use the blocking network API from the standard library?
    // Being single threaded and incapable of handling multiple connections is
    // probably a security feature.

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
            }
        };

        // The handle_stream function will async-block for the duration of
        // the connection which means that only one client can connect to
        // the server at a time. This is by design. We don't want multiple
        // clients.
        println!("Connected to {}", addr);
        match socket_ctx.handle_stream(stream).await {
            Ok(()) => println!("Disconnected"),
            Err(e) => println!("Disconnected: {}", e),
        }
    }
}
