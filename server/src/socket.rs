use futures::StreamExt;
use tokio::{sync::mpsc, net::TcpStream};
use tokio_tungstenite::tungstenite::Message;
use std::{
    net::SocketAddr,
    sync::{Arc, atomic::{AtomicBool, Ordering}}
};

#[derive(Clone)]
pub struct SocketContext {
    event: mpsc::UnboundedSender<tfc::Command>,
    connected: Arc<AtomicBool>,
}

impl SocketContext {
    pub fn new(event: mpsc::UnboundedSender<tfc::Command>) -> Self {
        Self {
            event,
            connected: Arc::new(AtomicBool::new(false))
        }
    }

    pub async fn connect(&self, stream: TcpStream, addr: SocketAddr) {
        // TODO: Not sure if this is still an issue
        // I think we need to manually send pings or something
        // https://github.com/seanmonstar/warp/issues/798

        if self.connected.fetch_or(true, Ordering::Acquire) {
            println!("Refused connection from: {}", addr);
            return;
        }

        let ws_stream = match tokio_tungstenite::accept_async(stream).await {
            Ok(s) => s,
            Err(e) => {
                self.connected.store(false, Ordering::Release);
                eprintln!("{}", e);
                return;
            }
        };
        let (_, mut ws_rx) = ws_stream.split();

        println!("Connected to: {}", addr);

        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(message) => self.receive(message),
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }

        println!("Disconnected from: {}", addr);

        self.connected.store(false, Ordering::Release);
    }

    fn receive(&self, message: Message) {
        if !message.is_binary() {
            return;
        }

        let bytes = message.into_data();
        let mut bytes = bytes.as_slice();
        if bytes.is_empty() {
            return;
        }

        loop {
            let (command, len) = match tfc::Command::from_bytes(bytes) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            };

            if self.event.send(command).is_err() {}
            if len < bytes.len() {
                bytes = &bytes[len..];
            } else {
                break;
            }
        }
    }
}
