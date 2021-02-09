mod filters;
mod handlers;

use warp::Filter;
use tokio::sync::mpsc;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let mut tfc_ctx = tfc::Context::new().unwrap();
    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<tfc::Command>();
    let ctx = handlers::SocketContext::new(ch_tx);

    pretty_env_logger::init();

    let routes = //filters::click()
        //.or(filters::downup())
        //.or(filters::press())
        //.or(filters::trackpad())
        filters::trackpad()
        .or(filters::slide())
        .or(filters::number())
        .or(filters::wasd())
        .or(filters::socket(ctx))
        .or(filters::js())
        .or(filters::css());

    tokio::spawn(async {
        warp::serve(routes.with(warp::log("tfc")))
            .run(([0, 0, 0, 0], 80))
            .await;
    });

    while let Some(command) = ch_rx.recv().await {
        tfc_ctx.execute_command(command).unwrap();
    }
}
