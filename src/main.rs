mod config;
mod redirect;
mod tracer;

use std::net::SocketAddr;

use anyhow::Context;
use axum::{extract::ConnectInfo, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use redirect::redirect_http_to_https;
use tracer::init_tracing_subscriber;

use crate::config::Config;

pub async fn root(ConnectInfo(info): ConnectInfo<SocketAddr>) {
    dbg!(info);
}

pub async fn run(app: Router<()>, config: Config) -> anyhow::Result<()> {
    let addr = config.get_addr();

    let tls_config = RustlsConfig::from_pem_file(config.cert_path(), config.cert_key_path())
        .await
        .with_context(|| "couldn't find PEM certificate")?;

    tracing::info!(
        "certificate and key loaded from path {} and {}",
        config.cert_path().display(),
        config.cert_key_path().display()
    );

    tracing::info!(%addr, "serving");

    let server = axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    if let Err(e) = server.await {
        tracing::info!("error running server: {e}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_path("example.env")?;

    let config = Config::try_from_env()?;

    let _guard = init_tracing_subscriber()?;

    tokio::spawn(redirect_http_to_https(config.ports()));

    let app = Router::<()>::new().route("/", get(root));

    run(app, config).await
}
