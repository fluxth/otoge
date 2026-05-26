mod extractors;
mod traits;

mod chunithm;
mod maimai;
mod ongeki;
mod polarischord;
mod popnmusic;
mod soundvoltex;

use std::path::Path;

use otoge::helpers::load_local_data_store;
use traits::{Extractor, FetchTask};

use otoge::chunithm::{ChunithmIntl, ChunithmJP};
use otoge::maimai::{MaimaiIntl, MaimaiJP};
use otoge::ongeki::Ongeki;
use otoge::polarischord::PolarisChord;
use otoge::popnmusic::PopNMusic;
use otoge::shared::traits::{DataStore as DataStoreTrait, Otoge};
use otoge::soundvoltex::SoundVoltex;

use anyhow::{Error, Result};
use tokio::task::JoinSet;
use tracing::metadata::LevelFilter;
use tracing::{Instrument, error, info, info_span, warn};
use tracing_subscriber::EnvFilter;

const DATA_PATH: &str = "./data";
const DEFAULT_USER_AGENT: &str = include_str!("./default_user_agent.txt");

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

    let client = reqwest::Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .build()?;

    let mut joinset: JoinSet<(&'static str, Result<()>)> = JoinSet::new();
    joinset.spawn(run::<SoundVoltex>(client.clone()));
    joinset.spawn(run::<PopNMusic>(client.clone()));
    joinset.spawn(run::<ChunithmJP>(client.clone()));
    joinset.spawn(run::<ChunithmIntl>(client.clone()));
    joinset.spawn(run::<Ongeki>(client.clone()));
    joinset.spawn(run::<MaimaiJP>(client.clone()));
    joinset.spawn(run::<MaimaiIntl>(client.clone()));
    joinset.spawn(run::<PolarisChord>(client));

    let mut return_result = Ok(());

    while let Some(join_result) = joinset.join_next().await {
        let (name, result) = join_result.expect("task panicked");
        if let Err(err) = result {
            error!("Task {} failed: {}", name, err);
            return_result = Err(Error::msg("One or more tasks failed"));
        } else {
            info!("Task {} succeeded", name);
        }
    }

    info!("All fetch completed");
    info!("Exiting");
    return_result
}

async fn run<G>(client: reqwest::Client) -> (&'static str, Result<()>)
where
    G: Otoge + FetchTask<G>,
    G::Extractor: Extractor<G>,
    G::Song: serde::de::DeserializeOwned + std::convert::From<G::ApiSong>,
    G::ApiSong: serde::de::DeserializeOwned,
    G::DataStore: DataStoreTrait + serde::de::DeserializeOwned + serde::Serialize,
{
    (G::name(), process::<G>(client).await)
}

async fn process<G>(client: reqwest::Client) -> Result<()>
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
        .unwrap_or_else(|err| {
            let span = info_span!("load_local", name);
            let _span = span.enter();

            warn!(error = %err, "Local song list not found or couldn't be loaded");
            None
        });

    let new_data_store = fetch_remote::<G>(&client)
        .instrument(info_span!("fetch_remote", name))
        .await?;

    G::verify_categories(&client, &new_data_store).await?;

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

async fn fetch_remote<G>(client: &reqwest::Client) -> Result<G::DataStore>
where
    G: Otoge + FetchTask<G>,
    G::Extractor: Extractor<G>,
{
    info!("Fetching remote song list");

    let songs = G::Extractor::fetch_songs(client).await?;
    info!("Fetched {} songs", &songs.len());

    Ok(G::new_data_store(songs))
}
