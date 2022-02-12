use crate::HOME;

use anyhow::{anyhow, Result};

/// Copy source configuration directories to destination directories.
pub(super) async fn copy_directory(path: &str, directory: &str) -> Result<()> {
    // Set source directory path.
    let source = format!("{path}\\{directory}");

    // Copy files from source to destination.
    let mut files = tokio::fs::read_dir(&source).await?;
    while let Some(file) = files.next_entry().await? {
        let destination = file
            .path()
            .to_str()
            .ok_or_else(|| anyhow!("Could not get file path."))?
            .split('\\')
            .rev()
            .next()
            .ok_or_else(|| anyhow!("Could not get file name."))?
            .to_string();

        // TODO: Will be `{HOME}/{directory}/{destination}` when Bystry fixes issue.
        let destination = format!("{HOME}\\legacy\\{directory}\\{destination}");
        tokio::fs::copy(file.path(), &destination).await?;
        info!("Installed file: '{destination}'.")
    }
    Ok(())
}
