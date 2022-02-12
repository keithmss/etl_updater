use crate::HOME;

mod utility;

use anyhow::{anyhow, Result};
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

#[rustfmt::skip]
const REPOSITORY: &str = "https://github.com/BystryPL/Legacy-Competition-League-Configs/archive/refs/heads/main.zip";
const REPOSITORY_NAME: &str = "Legacy-Competition-League-Configs-main";

/// Locate and download the latest competitive configurations.
pub(crate) async fn download() -> Result<String> {
    // Determine the destination file name.
    let destination = REPOSITORY
        .split('/')
        .rev()
        .next()
        .ok_or_else(|| anyhow!("Could not get file name."))?;

    // Download the snapshot.
    info!("Downloading: '{REPOSITORY}'...");
    let response = reqwest::get(REPOSITORY).await?;
    let content = response.bytes().await?;
    let mut file = tokio::fs::File::create(&destination).await?;
    file.write_all(&content).await?;
    info!("Saved file to: '{destination}'.");
    Ok(destination.to_string())
}

/// Install the configs.
pub(crate) async fn install(path: &str) -> Result<()> {
    info!("Installing configs and mapscripts to: '{HOME}'...");

    // Copy the configs.
    // TODO: Will be `legacy/configs` when Bystry fixes issue.
    utility::copy_directory(path, "configs").await?;

    // Copy the mapscripts.
    // TODO: Will be `legacy/mapscripts` when Bystry fixes issue.
    utility::copy_directory(path, "mapscripts").await?;

    // Remove unpacked files.
    tokio::fs::remove_dir_all(path).await?;

    info!("Installed configs.");
    Ok(())
}

/// Unpack the configs to the current working directory.
pub(crate) async fn unpack(path: String) -> Result<String> {
    info!("Unpacking: '{path}'...");
    let unpack = tokio::task::spawn_blocking(move || {
        // Unpack the archive.
        let zip = std::fs::File::open(&path)?;
        let mut archive = ZipArchive::new(zip)?;
        for index in 0..archive.len() {
            let mut file = archive.by_index(index)?;
            let destination = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };
            if (*file.name()).ends_with('/') {
                std::fs::create_dir_all(&destination)?;
            } else {
                if let Some(parent) = destination.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(&parent)?;
                    }
                }
                let mut output = std::fs::File::create(&destination)?;
                std::io::copy(&mut file, &mut output)?;
            }
        }

        // Remove the archive file.
        std::fs::remove_file(&path)?;

        // Return the path to the unpacked data.
        // TODO: Just derive the unpacked path and remove REPOSITORY_NAME.
        let path = REPOSITORY_NAME.to_string();
        info!("Unpacked to: '{path}'.");
        Ok(path)
    });
    unpack.await?
}
