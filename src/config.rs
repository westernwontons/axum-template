use std::{
    env,
    net::{IpAddr, SocketAddr},
    path::PathBuf
};

use anyhow::Context;

use crate::redirect::Ports;

/// App wide configuration provider
pub struct Config {
    app_host: IpAddr,
    http_port: u16,
    https_port: u16,
    cert_path: PathBuf,
    cert_key_path: PathBuf
}

impl Config {
    pub fn try_from_env() -> anyhow::Result<Self> {
        let app_host = env::var("APP_HOST").unwrap_or("127.0.0.1".into()).parse::<IpAddr>()?;
        let http_port = env::var("HTTP_PORT").unwrap_or("80".into()).parse::<u16>()?;
        let https_port = env::var("HTTPS_PORT").unwrap_or("443".into()).parse::<u16>()?;
        let cert_path =
            PathBuf::from(env::var("CERT_PATH").with_context(|| "CERT_PATH env var missing")?)
                .into();
        let cert_key_path = PathBuf::from(
            env::var("CERT_KEY_PATH").with_context(|| "CERT_KEY_PATH env var missing")?
        )
        .into();

        Ok(Self { app_host, cert_path, cert_key_path, http_port, https_port })
    }

    /// Returns a [`SocketAddr`] from host and port.
    pub fn get_addr(&self) -> SocketAddr {
        SocketAddr::new(self.app_host(), self.https_port())
    }

    /// Returns the app host of this [`Config`].
    pub fn app_host(&self) -> IpAddr {
        self.app_host
    }

    /// Returns the cert path of this [`Config`].
    pub fn cert_path(&self) -> &PathBuf {
        &self.cert_path
    }

    /// Returns the cert key path of this [`Config`].
    pub fn cert_key_path(&self) -> &PathBuf {
        &self.cert_key_path
    }

    pub fn http_port(&self) -> u16 {
        self.http_port
    }

    pub fn https_port(&self) -> u16 {
        self.https_port
    }

    pub fn ports(&self) -> Ports {
        Ports { http: self.http_port, https: self.https_port }
    }
}
