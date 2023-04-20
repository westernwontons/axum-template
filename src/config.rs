#![allow(dead_code)]

use std::{
    env,
    net::{IpAddr, SocketAddr},
    path::PathBuf
};

use anyhow::Context;

/// App wide configuration provider
pub struct Config {
    app_host: IpAddr,
    app_port: u16,
    cert_path: PathBuf,
    cert_key_path: PathBuf
}

impl Config {
    pub fn try_from_env() -> anyhow::Result<Self> {
        let app_host = env::var("APP_HOST").unwrap_or("127.0.0.1".into()).parse::<IpAddr>()?;
        let app_port = env::var("APP_PORT").unwrap_or("3000".into()).parse::<u16>()?;
        let cert_path =
            PathBuf::from(env::var("CERT_PATH").with_context(|| "CERT_PATH env var missing")?)
                .into();
        let cert_key_path = PathBuf::from(
            env::var("CERT_KEY_PATH").with_context(|| "CERT_KEY_PATH env var missing")?
        )
        .into();

        Ok(Self { app_host, app_port, cert_path, cert_key_path })
    }

    /// Returns a [`SocketAddr`] from host and port.
    pub fn get_addr(&self) -> SocketAddr {
        SocketAddr::new(self.app_host(), self.app_port())
    }

    /// Returns the app host of this [`Config`].
    pub fn app_host(&self) -> IpAddr {
        self.app_host
    }

    /// Returns the app port of this [`Config`].
    pub fn app_port(&self) -> u16 {
        self.app_port
    }
}
