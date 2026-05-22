mod traits;

use std::path::Path;

use anyhow::{Error, Result, anyhow};
use otoge::helpers::load_local_data_store;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::task::JoinSet;
use tracing::metadata::LevelFilter;
use tracing::{Instrument, error, info, info_span};
use tracing_subscriber::EnvFilter;

use otoge::chunithm::{ChunithmIntl, ChunithmJP};
use otoge::maimai::{MaimaiIntl, MaimaiJP};
use otoge::ongeki::Ongeki;
use otoge::popnmusic::PopNMusic;
use otoge::shared::traits::{DataStore, Otoge};
use otoge::soundvoltex::SoundVoltex;
use traits::GenerateTask;

const DATA_PATH: &str = "./data";
const GENERATED_PATH: &str = "./generated";

impl GenerateTask<Self> for ChunithmJP {}
impl GenerateTask<Self> for ChunithmIntl {}
impl GenerateTask<Self> for Ongeki {}
impl GenerateTask<Self> for MaimaiJP {}
impl GenerateTask<Self> for MaimaiIntl {}
impl GenerateTask<Self> for PopNMusic {}
impl GenerateTask<Self> for SoundVoltex {}

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

    let mut joinset: JoinSet<(&'static str, Result<()>)> = JoinSet::new();
    joinset.spawn(run::<SoundVoltex>());
    joinset.spawn(run::<PopNMusic>());
    joinset.spawn(run::<ChunithmJP>());
    joinset.spawn(run::<ChunithmIntl>());
    joinset.spawn(run::<Ongeki>());
    joinset.spawn(run::<MaimaiJP>());
    joinset.spawn(run::<MaimaiIntl>());

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

    info!("All generate tasks completed");
    info!("Exiting");
    return_result
}

async fn run<G>() -> (&'static str, Result<()>)
where
    G: Otoge + GenerateTask<G>,
    G::DataStore: DataStore + DeserializeOwned + Serialize,
{
    (G::name(), process::<G>().await)
}

async fn process<G>() -> Result<()>
where
    G: Otoge + GenerateTask<G>,
    G::DataStore: DataStore + DeserializeOwned + Serialize,
{
    let name = G::name();
    let data_path = Path::new(DATA_PATH);

    let local_data_store = load_local_data_store::<G>(Some(data_path))
        .instrument(info_span!("load_local", name))
        .await?;

    async {
        if let Some(data_store) = local_data_store {
            let generated_dir = Path::new(GENERATED_PATH);
            let music_dir = generated_dir.join("music");

            tokio::fs::create_dir_all(&music_dir).await?;

            let out_path = music_dir.join(format!("{name}.json"));

            info!("Generating music index");
            let json_content = serde_json::to_string(&data_store)?;

            info!("Saving output to {:?}", &out_path);
            tokio::fs::write(&out_path, &json_content).await?;

            Ok(())
        } else {
            let msg = "Could not find music data store";

            error!(msg);
            Err(anyhow!(msg))
        }
    }
    .instrument(info_span!("write_output", name))
    .await
}
