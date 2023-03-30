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
    format!("{} Hello, world: HOST={}, LocalAddr={}, RemoteAddr={}\n", local_datetime, server, get_ip_list()[0], addr)
}

// https://qiita.com/mm_sys/items/e3a62ddb3f8dbeee9031
#[cfg(target_os = "windows")]
fn get_ip_list() -> Vec<String> {
    use ipconfig;
    let mut ips: Vec<String> = Vec::new();
    match ipconfig::get_adapters() {
        Ok(adapters) => {
            for adapter in adapters {
                if adapter.oper_status() == ipconfig::OperStatus::IfOperStatusUp {
                    for address in adapter.ip_addresses() {
                        if !address.is_loopback() && address.is_ipv4() {
                            ips.push(address.to_string());
                        }
                    }
                }
            }
        }
        _ => {}
    }
    ips
}

#[cfg(not(target_os = "windows"))]
fn get_ip_list() -> Vec<String> {
    use pnet::datalink;
    let mut ips: Vec<String> = Vec::new();
    for interface in datalink::interfaces() {
        if !interface.ips.is_empty() && interface.is_up() {
            for ip_net in interface.ips {
                if ip_net.is_ipv4() && !ip_net.ip().is_loopback() {
                    ips.push(ip_net.ip().to_string());
                }
            }
        }
    };
    ips
}
