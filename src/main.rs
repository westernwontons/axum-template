use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{extract::ConnectInfo, routing::get, Router, Server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes the tracing subscriber with default values
#[tracing::instrument]
pub fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=info,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub async fn root(ConnectInfo(info): ConnectInfo<SocketAddr>) {
    dbg!(info);
}

pub async fn run(app: Router<()>) {
    let host = env::var("APP_HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("APP_PORT").unwrap_or("3000".into());

    let address = SocketAddr::new(
        host.parse()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        port.parse().unwrap_or(3000),
    );

    let server =
        Server::bind(&address).serve(app.into_make_service_with_connect_info::<SocketAddr>());

    if let Err(_) = server.await {
        println!("server error");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_tracing_subscriber();

    let app = Router::<()>::new().route("/", get(root));

    run(app).await;
}
