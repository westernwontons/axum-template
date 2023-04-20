#![allow(dead_code)]

use std::{env, io};

use anyhow::Context;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize a `tracing subscriber`
///
/// Will check for the presence of the `LOG` environment variable. It parses into a boolean.
/// If it's false, logs will be sent to the terminal, otherwise they go to the configured file.
///
/// The default path for the logs is the directory where the server was run from. It's recommended
/// to send them to somewhere like `/var/log`.
pub fn init_tracing_subscriber() -> anyhow::Result<WorkerGuard> {
    let log_to_file = env::var("LOG").unwrap_or("false".into()).parse::<bool>().unwrap_or(false);

    let log_dir = env::var("LOG_DIR").unwrap_or(".".into());

    let logfile_prefix = env::current_exe()?
        .file_name()
        .with_context(|| "failed to get name of current executable")?
        .to_str()
        .unwrap_or("tradingview-webhook-logs")
        .to_string();

    let (non_blocking, guard) = if log_to_file {
        let file_appender = tracing_appender::rolling::daily(log_dir, logfile_prefix);
        tracing_appender::non_blocking(file_appender)
    } else {
        tracing_appender::non_blocking(io::stdout())
    };

    let subscriber = tracing_subscriber::fmt::layer().with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=info,info".into())
        )
        .with(subscriber)
        .init();

    Ok(guard)
}
