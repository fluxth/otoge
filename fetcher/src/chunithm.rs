use std::collections::HashSet;

use anyhow::{ensure, Result};
use otoge::chunithm::models::{DataStore, Song, SongFromAPI};

use super::Otoge;

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

pub struct ChunithmJP;

impl Chunithm for ChunithmJP {}
impl Otoge for ChunithmJP {
    type DataStore = DataStore;
    type Song = Song;
    type ApiSong = SongFromAPI;

    fn name() -> &'static str {
        "chunithm_jp"
    }

    fn api_url() -> &'static str {
        "https://chunithm.sega.jp/storage/json/music.json"
    }

    fn verify_categories(data_store: &Self::DataStore) -> Result<()> {
        Self::impl_verify_categories(data_store)
    }

    fn new_data_store(songs: Vec<Self::Song>) -> Self::DataStore {
        DataStore::new(Self::name(), songs)
    }
}

pub struct ChunithmIntl;

impl Chunithm for ChunithmIntl {}
impl Otoge for ChunithmIntl {
    type DataStore = DataStore;
    type Song = Song;
    type ApiSong = SongFromAPI;

    fn name() -> &'static str {
        "chunithm_intl"
    }

    fn api_url() -> &'static str {
        "https://chunithm.sega.com/assets/data/music.json"
    }

    fn verify_categories(data_store: &Self::DataStore) -> Result<()> {
        Self::impl_verify_categories(data_store)
    }

    fn new_data_store(songs: Vec<Song>) -> DataStore {
        DataStore::new(Self::name(), songs)
    }
}
