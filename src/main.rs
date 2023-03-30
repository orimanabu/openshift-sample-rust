use axum::{
    response::IntoResponse,
    extract::ConnectInfo,
    routing::get,
    Router,
};
use axum_macros::debug_handler;
// use hyper::{header::HOST, Request, StatusCode};
use hyper::{header::HOST, Request};
use std::net::SocketAddr;

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

// impl Default for ConnectInfo {
//     fn default() -> Self {
//         Self { page: 1, per_page: 30 }
//     }
// }

// async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
//     format!("Hello {}", addr)
// }

// async fn handler(req: Request<hyper::Body>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Result<String, StatusCode> {
// async fn handler(req: Request<hyper::Body>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
// async fn handler(req: Request<hyper::Body>, conninfo: Option<ConnectInfo<SocketAddr>>) -> String {
#[debug_handler]
// async fn handler(req: Request<hyper::Body>, conninfo: Option<ConnectInfo<SocketAddr>>) -> impl IntoResponse {
async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: Request<hyper::Body>) -> impl IntoResponse {
    // let ConnectInfo(addr) = conninfo.unwrap_or_default();
    // let ConnectInfo(addr) = conninfo.unwrap();

    let server = match req.headers().get(HOST) {
        Some(header) => header.to_str().unwrap(),
        None => "Unknown",
    };

    format!("Hello from {} to {}", server, addr)
    // print!("Hello from {} to {}", server, addr);
}
