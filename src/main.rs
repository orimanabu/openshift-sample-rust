use axum::{
    response::IntoResponse,
    extract::ConnectInfo,
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use hyper::{header::HOST, Request};
use std::net::SocketAddr;
use chrono::{Local, DateTime};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(
            app.into_make_service_with_connect_info::<SocketAddr>()
        )
        .await
        .expect("server failed");
}

#[debug_handler]
async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: Request<hyper::Body>) -> impl IntoResponse {
    let server = match req.headers().get(HOST) {
        Some(header) => header.to_str().unwrap(),
        None => "Unknown",
    };

    let local_datetime: DateTime<Local> = Local::now();
    format!("{} Hello, world: from {} to {}\n", local_datetime, server, addr)
}
