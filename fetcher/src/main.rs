mod extractors;
mod traits;

mod chunithm;
mod maimai;
mod ongeki;
mod soundvoltex;

use std::path::Path;

use traits::{Extractor, FetchTask};

use otoge::chunithm::{ChunithmIntl, ChunithmJP};
use otoge::maimai::{MaimaiIntl, MaimaiJP};
use otoge::ongeki::Ongeki;
use otoge::shared::traits::{DataStore as DataStoreTrait, Otoge};
use otoge::soundvoltex::SoundVoltex;

use anyhow::{Error, Result};
use tokio::join;
use tracing::metadata::LevelFilter;
use tracing::{error, info, info_span, warn, Instrument};
use tracing_subscriber::EnvFilter;

const DATA_PATH: &str = "./data";

macro_rules! handle_result {
    ($index:tt, $type:ident, $results:ident, $return:ident) => {
        let name = $type::name();
        if let Err(err) = $results.$index {
            error!("Task {} failed: {}", name, err);
            $return = Err(Error::msg("One or more tasks failed"));
        } else {
            info!("Task {} succeeded", name);
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let format = tracing_subscriber::fmt::format().with_target(false);
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(filter)
        .init();

    info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // FIXME: Find a better way to do this :(
    let results = join!(
        process::<SoundVoltex>(),
        process::<ChunithmJP>(),
        process::<ChunithmIntl>(),
        process::<Ongeki>(),
        process::<MaimaiJP>(),
        process::<MaimaiIntl>(),
    );

    info!("All fetch completed");

    let mut return_result = Ok(());

    handle_result!(0, SoundVoltex, results, return_result);
    handle_result!(1, ChunithmJP, results, return_result);
    handle_result!(2, ChunithmIntl, results, return_result);
    handle_result!(3, Ongeki, results, return_result);
    handle_result!(4, MaimaiJP, results, return_result);
    handle_result!(5, MaimaiIntl, results, return_result);

    info!("Exiting");
    return_result
}

async fn process<G>() -> Result<()>
where
    G: Otoge + FetchTask<G>,
    G::Extractor: Extractor<G>,
    G::Song: serde::de::DeserializeOwned + std::convert::From<G::ApiSong>,
    G::ApiSong: serde::de::DeserializeOwned,
    G::DataStore: DataStoreTrait + serde::de::DeserializeOwned + serde::Serialize,
{
    let name = G::name();

    let data_dir = G::data_path(Some(Path::new(DATA_PATH)));
    tokio::fs::create_dir_all(&data_dir).await?;

    let music_toml_path = data_dir.join("music.toml");

    let local_data_store = load_local::<G>(&music_toml_path)
        .instrument(info_span!("load_local", name))
        .await?;

    let new_data_store = fetch_remote::<G>()
        .instrument(info_span!("fetch_remote", name))
        .await?;

    G::verify_categories(&new_data_store)?;

    async {
        let should_update = if let Some(data_store) = local_data_store {
            if data_store.data_differs(&new_data_store) {
                warn!("Local data differs from API, updating...");
                true
            } else {
                false
            }
        } else {
            true
        };

        if should_update {
            info!("Writing new data to {:?}", &music_toml_path.as_os_str());
            let toml_content = toml::to_string(&new_data_store)?;
            tokio::fs::write(&music_toml_path, &toml_content).await?;
        } else {
            info!("Local song list already up-to-date");
        }

        info!("Done");

        Ok::<(), anyhow::Error>(())
    }
    .instrument(info_span!("save", name))
    .await?;

    Ok(())
}

async fn load_local<G>(music_toml_path: &Path) -> Result<Option<<G as Otoge>::DataStore>>
where
    G: Otoge,
    G::DataStore: DataStoreTrait + serde::de::DeserializeOwned,
{
    info!(
        "Loading local song list at {:?}",
        music_toml_path.as_os_str()
    );

    let local_data_store = read_songs_toml(music_toml_path).await.ok();
    if local_data_store.is_none() {
        warn!("Local song list not found or couldn't be loaded");
    }

    Ok(local_data_store)
}

async fn read_songs_toml<S>(file_path: &Path) -> Result<S>
where
    S: serde::de::DeserializeOwned,
{
    let contents = tokio::fs::read_to_string(file_path).await?;

    Ok(toml::from_str::<S>(contents.as_str())?)
}

async fn fetch_remote<G>() -> Result<G::DataStore>
where
    G: Otoge + FetchTask<G>,
    G::Extractor: Extractor<G>,
{
    info!("Fetching remote song list");

    let songs = G::Extractor::fetch_songs().await?;
    info!("Fetched {} songs", &songs.len());

    Ok(G::new_data_store(songs))
}
