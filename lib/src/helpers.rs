use anyhow::Result;
use std::path::Path;
use tracing::info;

use crate::shared::traits::{DataStore, Otoge};

pub async fn load_local_data_store<G>(
    base_path: Option<&Path>,
) -> Result<Option<<G as Otoge>::DataStore>>
where
    G: Otoge,
    G::DataStore: DataStore + serde::de::DeserializeOwned,
{
    let music_data_store_path = G::music_data_store_path(base_path);

    info!(
        "Loading local song list at {:?}",
        music_data_store_path.as_os_str()
    );

    let local_data_store = read_music_toml(music_data_store_path.as_path()).await?;
    Ok(local_data_store)
}

async fn read_music_toml<S>(file_path: &Path) -> Result<S>
where
    S: serde::de::DeserializeOwned,
{
    let contents = tokio::fs::read_to_string(file_path).await?;

    Ok(toml::from_str::<S>(contents.as_str())?)
}
