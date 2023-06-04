use std::collections::HashSet;

use otoge::chunithm::models::{Category, DataStore, Song, SongFromAPI};

use anyhow::{ensure, Error, Result};
use tokio::join;

const CHUNITHM_JP_API_URL: &str = "https://chunithm.sega.jp/storage/json/music.json";
const CHUNITHM_INTL_API_URL: &str = "https://chunithm.sega.com/assets/data/music.json";

const DATA_PATH: &str = "./data";

#[tokio::main]
async fn main() -> Result<()> {
    println!("[main] Starting up...");

    let (chunithm_jp, chunithm_intl) = join!(
        process("chunithm_jp", CHUNITHM_JP_API_URL),
        process("chunithm_intl", CHUNITHM_INTL_API_URL)
    );

    println!("[main] All fetch completed");

    let mut return_result = Ok(());
    for (name, result) in [
        ("chunithm_jp", chunithm_jp),
        ("chunithm_intl", chunithm_intl),
    ] {
        if let Err(err) = result {
            println!("[main] Task {} failed: {}", name, err);
            return_result = Err(Error::msg("One or more tasks failed"));
        } else {
            println!("[main] Task {} succeeded", name);
        }
    }

    return_result
}

async fn process(name: &'static str, api_url: &str) -> Result<()> {
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

    verify_categories(&new_data_store.categories, &new_data_store.songs)?;

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

async fn read_songs_toml(file_path: &str) -> Result<DataStore> {
    let contents = tokio::fs::read_to_string(file_path).await?;

    Ok(toml::from_str::<DataStore>(contents.as_str())?)
}

fn verify_categories(categories: &[Category], songs: &[Song]) -> Result<()> {
    let all_categories: HashSet<&str> = categories.iter().map(|c| c.name.as_ref()).collect();

    let mut song_categories = HashSet::<&str>::new();
    for song in songs {
        song_categories.insert(song.category.as_str());
    }

    let diff_from_categories = all_categories.difference(&song_categories);
    let diff_from_songs = song_categories.difference(&all_categories);

    ensure!(
        diff_from_categories.clone().count() == 0 && diff_from_songs.clone().count() == 0,
        "Local category definitions differs, +{:?} -{:?}",
        diff_from_categories,
        diff_from_songs
    );

    Ok(())
}

async fn fetch_songs(url: &str) -> Result<Vec<Song>> {
    let resp = reqwest::get(url).await?;
    let data = resp.json::<Vec<SongFromAPI>>().await?;

    Ok(data.into_iter().map(|song| song.into()).collect())
}
