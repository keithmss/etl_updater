mod utility;

use crate::HOME;

use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use scraper::{Html, Selector};
use tar::Archive;
use tokio::io::AsyncWriteExt;

const TARGET: &str = "x86_64.tar.gz";
const WORKFLOW: &str = "https://www.etlegacy.com/workflow-files";

/// Locate and download the latest snapshot from the ETL workflow site.
pub(crate) async fn download(url: String) -> Result<String> {
    // Determine the destination file name.
    let destination = url
        .split('/')
        .rev()
        .next()
        .ok_or_else(|| anyhow!("Could not get file name."))?;

    // Download the snapshot.
    info!("Downloading: '{url}'...");
    let response = reqwest::get(&url).await?;
    let content = response.bytes().await?;
    let mut file = tokio::fs::File::create(&destination).await?;
    file.write_all(&content).await?;
    info!("Saved file to: '{destination}'.");
    Ok(destination.to_string())
}

/// Retrieve the url to the latest snapshot.
pub(crate) async fn fetch_url() -> Result<String> {
    // Retrieve website data.
    info!("Fetching latest snapshot information...");
    let response = reqwest::get(WORKFLOW).await?;
    let content = response.text().await?;

    // Locate the desired snapshot.
    // TODO: This will always just fetch the first which happens to be the
    //       latest. At some point this should be a more comprehensive check.
    let document = Html::parse_document(&content);
    let selector = Selector::parse("a").map_err(|_| anyhow!("Could not parse html."))?;
    let url = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .find(|url| url.ends_with(TARGET))
        .ok_or_else(|| anyhow!("Could not find target file ending with: '{TARGET}'."))?;
    Ok(url.to_string())
}

/// Install the snapshot.
pub(crate) async fn install(path: &str) -> Result<()> {
    // Clean existing directory.
    utility::try_clean_pk3().await?;

    info!("Installing snapshot to: '{HOME}'...");
    // Copy the etl binary.
    utility::copy_file(path, "etl").await?;

    // Copy the etlded binary.
    utility::copy_file(path, "etlded").await?;

    // Copy the renderer binary.
    utility::copy_file(path, "librenderer_opengl1_x86_64.so").await?;

    // Copy pk3 file (with some massaging).
    // TODO: Propose they make this file name consistent.
    let pk3 = path.replace("etlegacy-", "legacy_").replace("-x86_64", "");
    let pk3 = format!("legacy/{pk3}.pk3");
    utility::copy_file(path, &pk3).await?;

    // Copy qagame binary.
    utility::copy_file(path, "legacy/qagame.mp.x86_64.so").await?;

    // Remove unpacked files.
    tokio::fs::remove_dir_all(&path).await?;
    info!("Installed snapshot.");
    Ok(())
}

/// Unpack the snapshot to the current working directory.
pub(crate) async fn unpack(path: String) -> Result<String> {
    info!("Unpacking: '{path}'...");
    let unpack = tokio::task::spawn_blocking(move || {
        // Unpack the archive.
        let tar_gz = std::fs::File::open(&path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(".")?;

        // Remove the archive file.
        std::fs::remove_file(&path)?;

        // Return the path to the unpacked data.
        // TODO: Refactor later to make this handle different workflow files.
        let path = path
            .strip_suffix(".tar.gz")
            .ok_or_else(|| anyhow!("Could not get path."))?
            .to_string();
        info!("Unpacked to: '{path}'.");
        Ok(path)
    });
    unpack.await?
}
