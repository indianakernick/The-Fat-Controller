mod socket;

use std::net::TcpListener;

fn main() {
    let tfc_ctx = match tfc::Context::new() {
        Ok(c) => c,
        Err(e) => {
            println!("Initialize: {}", e);
            return;
        }
    };
    let mut socket_ctx = socket::SocketContext::new(tfc_ctx);

    let listener = match TcpListener::bind("0.0.0.0:80") {
        Ok(l) => l,
        Err(e) => {
            println!("Bind: {}", e);
            return;
        },
    };

    println!("Listening on port 80");

    loop {
        let (stream, addr) = match listener.accept() {
            Ok(s) => s,
            Err(e) => {
                println!("Accept: {}", e);
                continue;
            }
        };

        println!("Connected to {}", addr);
        match socket_ctx.handle_stream(stream) {
            Ok(()) => println!("Disconnected"),
            Err(e) => println!("Disconnected: {}", e),
        }
    }
}
