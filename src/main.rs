mod filters;
mod handlers;
mod socket_command;
mod macos;

use warp::Filter;
use tokio::sync::mpsc;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut event = macos::EventContext::default();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<macos::Command>();
    let ctx = handlers::SocketContext::new(ch_tx);

    pretty_env_logger::init();

    let routes = filters::click()
        .or(filters::downup())
        .or(filters::press())
        .or(filters::trackpad())
        .or(filters::slide())
        .or(filters::number())
        .or(filters::socket(ctx))
        .or(filters::js())
        .or(filters::css());

    tokio::spawn(async {
        warp::serve(routes.with(warp::log("key")))
            .run(([0, 0, 0, 0], 80))
            .await;
    });

    while let Some(command) = ch_rx.recv().await {
        event.evaluate_command(command);
    }
}
