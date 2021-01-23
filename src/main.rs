mod filters;
mod handlers;
mod enigo_command;
mod socket_command;

use warp::Filter;
use enigo::Enigo;
use tokio::sync::mpsc;
use crate::enigo_command::{EnigoCommand, parse_enigo_command};

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut enigo = Enigo::new();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<EnigoCommand>();

    let ctx = handlers::SocketContext::new(ch_tx);

    pretty_env_logger::init();

    let routes = filters::click()
        .or(filters::downup())
        .or(filters::press())
        .or(filters::trackpad())
        .or(filters::socket(ctx))
        .or(filters::js())
        .or(filters::css());

    tokio::spawn(async {
        warp::serve(routes.with(warp::log("key")))
            .run(([0, 0, 0, 0], 80))
            .await;
    });

    while let Some(command) = ch_rx.recv().await {
        parse_enigo_command(&mut enigo, command);
    }
}
