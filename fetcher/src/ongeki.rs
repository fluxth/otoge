use std::collections::HashSet;

use anyhow::{ensure, Result};

use crate::extractors::serde::SerdeExtractor;
use crate::traits::FetchTask;
use otoge::ongeki::models::{DataStore, Song, SongFromAPI};
use otoge::shared::traits::Otoge;

pub struct Ongeki;

impl Otoge for Ongeki {
    type DataStore = DataStore;
    type Song = Song;

    fn name() -> &'static str {
        "ongeki"
    }
}

impl FetchTask<Self> for Ongeki {
    type ApiSong = SongFromAPI;
    type Extractor = SerdeExtractor;

    fn api_url() -> &'static str {
        "https://ongeki.sega.jp/assets/json/music/music.json"
    }

    fn new_data_store(songs: Vec<Song>) -> DataStore {
        DataStore::new(Self::name(), songs)
    }

    fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        let categories = &data_store.categories;
        let songs = &data_store.songs;

        let all_categories: HashSet<&str> = categories.iter().map(|cat| cat.id.as_ref()).collect();

        let mut song_categories = HashSet::<&str>::new();
        for song in songs {
            song_categories.insert(song.category.id.as_ref());
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
