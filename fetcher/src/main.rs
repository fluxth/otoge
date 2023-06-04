use otoge::chunithm::models::{DataStore, Song, SongFromAPI};

use anyhow::Result;
use tokio::try_join;

const CHUNITHM_JP_API_URL: &'static str = "https://chunithm.sega.jp/storage/json/music.json";
const CHUNITHM_INTL_API_URL: &'static str = "https://chunithm.sega.com/assets/data/music.json";

const DATA_PATH: &'static str = "./data";

#[tokio::main]
async fn main() -> Result<()> {
    let results = try_join!(
        process("chunithm_jp", CHUNITHM_JP_API_URL),
        process("chunithm_intl", CHUNITHM_INTL_API_URL),
    );

    match results {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

async fn process(name: &str, api_url: &str) -> Result<()> {
    let data_dir = format!("{}/{}", DATA_PATH, name);
    tokio::fs::create_dir_all(&data_dir).await?;

    let music_toml_path = format!("{}/music.toml", data_dir);
    println!(
        "[{}] Loading local song list at '{}'",
        name, music_toml_path
    );

    let local_data_store = read_songs_toml(&music_toml_path).await.ok();
    if local_data_store.is_none() {
        println!("[{}] Local song list not found or couldn't be loaded", name);
    }

    println!("[{}] Fetching song list", name);

    let songs = fetch_songs(api_url).await?;
    println!("[{}] Fetched {} songs", name, &songs.len());

    let new_data_store = DataStore::new(name, songs);

    let mut should_update = false;

    if let Some(data_store) = local_data_store {
        if data_store.data_differs(&new_data_store) {
            println!("[{}] Local data differs from API, updating...", name);
            should_update = true;
        }
    } else {
        should_update = true;
    }

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

async fn read_songs_toml(file_path: &str) -> Result<DataStore> {
    let contents = tokio::fs::read_to_string(file_path).await?;

    Ok(toml::from_str::<DataStore>(contents.as_str())?)
}

async fn fetch_songs(url: &str) -> Result<Vec<Song>> {
    let resp = reqwest::get(url).await?;
    let data = resp.json::<Vec<SongFromAPI>>().await?;

    Ok(data.into_iter().map(|song| song.into()).collect())
}
