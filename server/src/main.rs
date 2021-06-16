mod socket;

use tokio::{sync::mpsc, net::TcpListener};

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut tfc_ctx = match tfc::Context::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<tfc::Command>();
    let sock_ctx = socket::SocketContext::new(ch_tx);

    tokio::spawn(async move {
        let listener = match TcpListener::bind("0.0.0.0:80").await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        while let Ok((stream, addr)) = listener.accept().await {
            let ctx = sock_ctx.clone();
            tokio::spawn(async move {
                ctx.connect(stream, addr).await
            });
        }
    });

    while let Some(command) = ch_rx.recv().await {
        if let Err(e) = command.execute_async(&mut tfc_ctx).await {
            eprintln!("{}", e);
        }
    }
}
