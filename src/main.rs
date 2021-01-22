mod filters;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = filters::root()
        .or(filters::js())
        .or(filters::css());

    warp::serve(routes.with(warp::log("key")))
        .run(([0, 0, 0, 0], 80))
        .await;
}
