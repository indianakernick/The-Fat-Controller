mod filters;
mod handlers;

use warp::Filter;

#[tokio::main]
async fn main() {
    let ctx = handlers::SocketContext::new();

    pretty_env_logger::init();

    let routes = filters::root()
        .or(filters::socket(ctx))
        .or(filters::js())
        .or(filters::css());

    warp::serve(routes.with(warp::log("key")))
        .run(([0, 0, 0, 0], 80))
        .await;
}
