mod chunithm;

use chunithm::{ChunithmIntl, ChunithmJP};

use otoge::shared::traits::DataStore as DataStoreTrait;

use anyhow::{Error, Result};
use tokio::join;

const DATA_PATH: &str = "./data";

pub trait Otoge {
    type DataStore;
    type Song;
    type ApiSong;

    fn name() -> &'static str;
    fn api_url() -> &'static str;

    fn new_data_store(songs: Vec<Self::Song>) -> Self::DataStore;
    fn verify_categories(_data_store: &Self::DataStore) -> Result<()> {
        Ok(())
    }
}

macro_rules! handle_result {
    ($index:tt, $type:ident, $results:ident, $return:ident) => {
        let name = $type::name();
        if let Err(err) = $results.$index {
            println!("[main] Task {} failed: {}", name, err);
            $return = Err(Error::msg("One or more tasks failed"));
        } else {
            println!("[main] Task {} succeeded", name);
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("[main] Starting up...");

    // FIXME: Find a better way to do this :(
    let results = join!(process::<ChunithmJP>(), process::<ChunithmIntl>(),);

    println!("[main] All fetch completed");

    let mut return_result = Ok(());

    handle_result!(0, ChunithmJP, results, return_result);
    handle_result!(1, ChunithmIntl, results, return_result);

    return_result
}

async fn process<G>() -> Result<()>
where
    G: Otoge,
    G::Song: serde::de::DeserializeOwned + std::convert::From<G::ApiSong>,
    G::ApiSong: serde::de::DeserializeOwned,
    G::DataStore: DataStoreTrait + serde::de::DeserializeOwned + serde::Serialize,
{
    let name = G::name();
    let api_url = G::api_url();

    let data_dir = format!("{}/{}", DATA_PATH, name);
    tokio::fs::create_dir_all(&data_dir).await?;

    let music_toml_path = format!("{}/music.toml", data_dir);
    println!(
        "[{}] Loading local song list at '{}'",
        name, music_toml_path
    );

    let local_data_store: Option<G::DataStore> = read_songs_toml(&music_toml_path).await.ok();
    if local_data_store.is_none() {
        println!("[{}] Local song list not found or couldn't be loaded", name);
    }

    println!("[{}] Fetching song list", name);

    let songs = fetch_songs::<G::ApiSong, G::Song>(api_url).await?;
    println!("[{}] Fetched {} songs", name, &songs.len());

    let new_data_store = G::new_data_store(songs);

    G::verify_categories(&new_data_store)?;

    let should_update = if let Some(data_store) = local_data_store {
        if data_store.data_differs(&new_data_store) {
            println!("[{}] Local data differs from API, updating...", name);
            true
        } else {
            false
        }
    } else {
        true
    };

    if should_update {
        println!("[{}] Writing new data to '{}'", name, &music_toml_path);
        let toml_content = toml::to_string(&new_data_store)?;
        tokio::fs::write(&music_toml_path, &toml_content).await?;
    } else {
        println!("[{}] Local song list already up-to-date", name);
    }

    println!("[{}] Done", name);
    Ok(())
}

async fn read_songs_toml<S>(file_path: &str) -> Result<S>
where
    S: serde::de::DeserializeOwned,
{
    let contents = tokio::fs::read_to_string(file_path).await?;

    Ok(toml::from_str::<S>(contents.as_str())?)
}

async fn fetch_songs<Api, Out>(url: &str) -> Result<Vec<Out>>
where
    Api: serde::de::DeserializeOwned,
    Out: std::convert::From<Api>,
{
    let resp = reqwest::get(url).await?;
    let data = resp.json::<Vec<Api>>().await?;

    Ok(data.into_iter().map(|song| song.into()).collect())
}
