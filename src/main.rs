#[macro_use]
extern crate tracing;

mod configs;
mod logger;
mod snapshot;

use anyhow::Result;

pub(crate) const HOME: &str = "/opt/etscrim";

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    // Spawn the task to download and install the latest snapshot.
    let snapshot_task = tokio::task::spawn(async { snapshot().await });

    // Spawn the task to download and install the latest competition configs.
    let configs_task = tokio::task::spawn(async { configs().await });

    // Await the completion of the snapshot task.
    if let Err(why) = snapshot_task.await? {
        error!("Error installing latest snapshot: '{why}'.");
    }

    // Await the completion of the configs task.
    if let Err(why) = configs_task.await? {
        error!("Error installing latest competition configs: '{why}'");
    }

    // All tasks have completed.
    Ok(())
}

// Retrieve and install the latest ETL competition configs.
async fn configs() -> Result<()> {
    // Download the latest configs.
    let configs = configs::download().await?;

    // Unpack the configs.
    let data = configs::unpack(configs).await?;

    // Install the configs.
    configs::install(&data).await?;
    Ok(())
}

// Retrieve and install the latest ETL snapshot.
async fn snapshot() -> Result<()> {
    // Retrieve the snapshot url.
    let url = snapshot::fetch_url().await?;

    // Download the snapshot.
    let snapshot = snapshot::download(url).await?;

    // Unpack the snapshot.
    let data = snapshot::unpack(snapshot).await?;

    // Install the snapshot.
    snapshot::install(&data).await?;
    Ok(())
}
