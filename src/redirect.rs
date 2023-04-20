#![allow(dead_code)]

use axum::extract::Host;
use axum::handler::HandlerWithoutStateExt;
use axum::http::{uri, StatusCode, Uri};
use axum::response::Redirect;
use axum::BoxError;

use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Ports {
    pub http: u16,
    pub https: u16
}

impl Ports {
    /// Creates a new [`Ports`].
    pub fn new(http: u16, https: u16) -> Self {
        Self { http, https }
    }
}

/// Redirect http to https
///
/// Stolen from: https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs
pub async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());

        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = {
        let ports = ports.clone();
        move |Host(host): Host, uri: Uri| async move {
            match make_https(host, uri, ports) {
                Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
                Err(error) => {
                    tracing::warn!(%error, "failed to convert URI to HTTPS");
                    Err(StatusCode::BAD_REQUEST)
                }
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));
    tracing::debug!("http redirect listening on {}", addr);

    axum::Server::bind(&addr).serve(redirect.into_make_service()).await.unwrap()
}
