use crate::HOME;

use anyhow::Result;

/// Copy source snapshot files to destination directories.
pub(super) async fn copy_file(directory: &str, file: &str) -> Result<()> {
    // Set source and destination directory paths.
    let source = format!("{directory}/{file}");
    let destination = format!("{HOME}/{file}");

    // Copy source file to destination.
    tokio::fs::copy(&source, &destination).await?;
    info!("Installed file: '{destination}'.");
    Ok(())
}

// Attempt to remove the existing pk3.
pub(super) async fn try_clean_pk3() -> Result<()> {
    info!("Attempting to delete existing pk3 files...");

    // Locate existing pk3's.
    let legacy = format!("{HOME}/legacy");
    let mut files = tokio::fs::read_dir(legacy).await?;
    while let Some(file) = files.next_entry().await? {
        let file = file.path().to_str().unwrap_or_default().to_string();
        if file.contains("legacy_v") && file.ends_with(".pk3") {
            tokio::fs::remove_file(&file).await?;
            info!("\tDeleted: '{file}'.")
        }
    }
    Ok(())
}
