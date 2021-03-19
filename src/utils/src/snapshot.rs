// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use log::{error, info, warn};
use std::{fs::File, io::copy, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    InvalidFilePath(String),
    #[error("")]
    NoDownloadSourceAvailable,
}

pub async fn bootstrap_balances_from_snapshot() {
    download_snapshot_file(Path::new("full_snapshot.bin"),
                           &[String::from("https://dbfiles.testnet.chrysalis2.com/")]).await.unwrap();
}

async fn download_snapshot_file(file_path: &Path, download_urls: &[String]) -> Result<(), Error> {
    let file_name = file_path
        .file_name()
        .ok_or_else(|| Error::InvalidFilePath(file_path.to_string_lossy().to_string()))?;

    std::fs::create_dir_all(
        file_path
            .parent()
            .ok_or_else(|| Error::InvalidFilePath(file_path.to_string_lossy().to_string()))?,
    )
        .map_err(|_| Error::InvalidFilePath(file_path.to_string_lossy().to_string()))?;

    for url in download_urls {
        let url = url.to_owned() + &file_name.to_string_lossy();

        info!("Downloading snapshot file {}...", url);
        match reqwest::get(&url).await {
            Ok(res) => match File::create(file_path) {
                // TODO unwrap
                Ok(mut file) => match copy(&mut res.bytes().await.unwrap().as_ref(), &mut file) {
                    Ok(_) => break,
                    Err(e) => warn!("Copying snapshot file failed: {:?}.", e),
                },
                Err(e) => warn!("Creating snapshot file failed: {:?}.", e),
            },
            Err(e) => warn!("Downloading snapshot file failed: {:?}.", e),
        }
    }

    if !file_path.exists() {
        error!("No working download source available.");
        return Err(Error::NoDownloadSourceAvailable);
    }

    Ok(())
}
