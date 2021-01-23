use warp::Filter;
use crate::handlers;
use std::convert::Infallible;

fn with_state<S: Clone + Send>(state: S) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn click() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("click" / String)
        .and(warp::get())
        .and_then(handlers::click)
}

pub fn socket(ctx: handlers::SocketContext) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("socket")
        .and(warp::ws())
        .and(with_state(ctx))
        .and_then(handlers::socket_upgrade)
}

pub fn js() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("js")
        .and(warp::get())
        .and(warp::fs::dir("./client/dist/js"))
}

pub fn css() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("css")
        .and(warp::get())
        .and(warp::fs::dir("./client/dist/css"))
}
