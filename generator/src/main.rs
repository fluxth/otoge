mod traits;

use std::path::Path;

use anyhow::{Error, Result, anyhow};
use otoge::helpers::load_local_data_store;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::join;
use tracing::metadata::LevelFilter;
use tracing::{Instrument, error, info, info_span};
use tracing_subscriber::EnvFilter;

use otoge::chunithm::{ChunithmIntl, ChunithmJP};
use otoge::maimai::{MaimaiIntl, MaimaiJP};
use otoge::ongeki::Ongeki;
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
impl GenerateTask<Self> for SoundVoltex {}

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

    info!("All generate tasks completed");

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

            let out_path = music_dir.join(format!("{}.json", name));

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
