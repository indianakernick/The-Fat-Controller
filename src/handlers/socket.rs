use log::error;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use futures::{FutureExt, StreamExt};
use crate::enigo_command::EnigoCommand;
use warp::ws::{Ws, WebSocket, Message};
use crate::socket_command::parse_socket_command;
use tokio_stream::wrappers::UnboundedReceiverStream;

type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

pub async fn socket_upgrade(ws: Ws, ctx: SocketContext) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    ctx.upgrade(ws).await
}

#[derive(Clone)]
pub struct SocketContext {
    ch_tx: Arc<RwLock<Option<Sender>>>,
    enigo: mpsc::UnboundedSender<EnigoCommand>,
}

impl SocketContext {
    pub fn new(enigo: mpsc::UnboundedSender<EnigoCommand>) -> Self {
        Self {
            ch_tx: Default::default(),
            enigo
        }
    }

    async fn upgrade(self, ws: Ws) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        // Sometimes we end up in a state where we are disconnected but
        // self.ch_tx is Some. Overriding any existing is a workaround for this.
        // if self.ch_tx.read().await.is_some() {
        //     return Ok(Box::new(warp::http::StatusCode::FORBIDDEN));
        // }

        Ok(Box::new(ws.on_upgrade(move |socket: WebSocket| {
            self.connect(socket)
        })))
    }

    async fn connect(self, ws: WebSocket) {
        let (ws_tx, mut ws_rx) = ws.split::<Message>();
        let (ch_tx, ch_rx) = mpsc::unbounded_channel::<Result<Message, warp::Error>>();
        let ch_rx = UnboundedReceiverStream::new(ch_rx);

        *self.ch_tx.write().await = Some(ch_tx);

        tokio::task::spawn(ch_rx.forward(ws_tx).map(move |result: Result<(), warp::Error>| {
            if let Err(e) = result {
                error!("Error sending over socket: {}", e);
            }
        }));

        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(message) => self.receive(message),
                Err(e) => {
                    error!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }

        *self.ch_tx.write().await = None;
    }

    fn receive(&self, message: Message) {
        if message.is_binary() {
            let mut bytes = message.as_bytes();
            loop {
                let (command, len) = parse_socket_command(bytes);
                if let EnigoCommand::Null = command {
                    return;
                }
                if self.enigo.send(command).is_err() {}
                if len < bytes.len() {
                    bytes = &bytes[len..];
                } else {
                    break;
                }
            }
        }
    }
}
