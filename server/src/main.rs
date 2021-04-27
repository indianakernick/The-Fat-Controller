mod socket;

use warp::Filter;
use tokio::sync::mpsc;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut tfc_ctx = tfc::Context::new().unwrap();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<tfc::Command>();
    let sock_ctx = socket::SocketContext::new(ch_tx);

    pretty_env_logger::init();

    let routes = warp::path::end()
        .and(warp::ws())
        .and(warp::any().map(move || sock_ctx.clone()))
        .and_then(socket::socket_upgrade);

    tokio::spawn(async {
        warp::serve(routes.with(warp::log("tfc")))
            .run(([0, 0, 0, 0], 80))
            .await;
    });

    while let Some(command) = ch_rx.recv().await {
        command.execute(&mut tfc_ctx).unwrap();
    }
}
