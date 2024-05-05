mod extractors;
mod traits;

mod chunithm;
mod maimai;
mod ongeki;
mod polarischord;
mod soundvoltex;

use std::path::Path;

use otoge::helpers::load_local_data_store;
use traits::{Extractor, FetchTask};

use otoge::chunithm::{ChunithmIntl, ChunithmJP};
use otoge::maimai::{MaimaiIntl, MaimaiJP};
use otoge::ongeki::Ongeki;
use otoge::polarischord::PolarisChord;
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
        process::<PolarisChord>(),
    );

    info!("All fetch completed");

    let mut return_result = Ok(());

    handle_result!(0, SoundVoltex, results, return_result);
    handle_result!(1, ChunithmJP, results, return_result);
    handle_result!(2, ChunithmIntl, results, return_result);
    handle_result!(3, Ongeki, results, return_result);
    handle_result!(4, MaimaiJP, results, return_result);
    handle_result!(5, MaimaiIntl, results, return_result);
    handle_result!(6, PolarisChord, results, return_result);

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

    let data_path = Path::new(DATA_PATH);
    let data_dir = G::data_path(Some(data_path));
    tokio::fs::create_dir_all(&data_dir).await?;

    let local_data_store = load_local_data_store::<G>(Some(data_path))
        .instrument(info_span!("load_local", name))
        .await
        .unwrap_or_else(|_err| {
            let span = info_span!("load_local", name);
            let _span = span.enter();

            warn!("Local song list not found or couldn't be loaded");
            None
        });

    let new_data_store = fetch_remote::<G>()
        .instrument(info_span!("fetch_remote", name))
        .await?;

    G::verify_categories(&new_data_store).await?;

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
            let music_data_store_path = G::music_data_store_path(Some(data_path));
            info!(
                "Writing new data to {:?}",
                &music_data_store_path.as_os_str()
            );
            let toml_content = toml::to_string(&new_data_store)?;
            tokio::fs::write(&music_data_store_path, &toml_content).await?;
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
