mod filters;
mod handlers;

use warp::Filter;
use tokio::sync::mpsc;
use enigo::{Enigo, Key, KeyboardControllable};

pub enum EnigoCommand {
    KeyClick(Key),
    KeyDown(Key),
    KeyUp(Key),
}

#[tokio::main]
async fn main() {
    let mut enigo = Enigo::new();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<EnigoCommand>();

    let ctx = handlers::SocketContext::new(ch_tx);

    pretty_env_logger::init();

    let routes = filters::root()
        .or(filters::socket(ctx))
        .or(filters::js())
        .or(filters::css());

    tokio::spawn(async {
        warp::serve(routes.with(warp::log("key")))
            .run(([0, 0, 0, 0], 80))
            .await;
    });

    while let Some(command) = ch_rx.recv().await {
        match command {
            EnigoCommand::KeyClick(key) => enigo.key_click(key),
            EnigoCommand::KeyDown(key) => enigo.key_down(key),
            EnigoCommand::KeyUp(key) => enigo.key_up(key),
        }
    }
}
