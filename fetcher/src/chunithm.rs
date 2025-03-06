use std::collections::HashSet;

use anyhow::{Result, ensure};
use async_trait::async_trait;

use otoge::chunithm::models::{DataStore, Song, SongFromAPI};
use otoge::chunithm::{ChunithmIntl, ChunithmJP};
use otoge::shared::traits::Otoge;

use crate::extractors::serde::SerdeGetExtractor;
use crate::traits::FetchTask;

trait Chunithm {
    fn impl_verify_categories(data_store: &DataStore) -> Result<()> {
        let categories = &data_store.categories;
        let songs = &data_store.songs;

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
}

impl Chunithm for ChunithmJP {}
#[async_trait]
impl FetchTask<Self> for ChunithmJP {
    type ApiSong = SongFromAPI;
    type Extractor = SerdeGetExtractor;

    fn api_url() -> &'static str {
        "https://chunithm.sega.jp/storage/json/music.json"
    }

    async fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        Self::impl_verify_categories(data_store)
    }

    fn new_data_store(songs: Vec<<Self as Otoge>::Song>) -> <Self as Otoge>::DataStore {
        DataStore::new(Self::name(), songs)
    }
}

impl Chunithm for ChunithmIntl {}
#[async_trait]
impl FetchTask<Self> for ChunithmIntl {
    type ApiSong = SongFromAPI;
    type Extractor = SerdeGetExtractor;

    fn api_url() -> &'static str {
        "https://chunithm.sega.com/assets/data/music.json"
    }

    async fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        Self::impl_verify_categories(data_store)
    }

    fn new_data_store(songs: Vec<Song>) -> DataStore {
        DataStore::new(Self::name(), songs)
    }
}
