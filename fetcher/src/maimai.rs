use std::collections::HashSet;

use anyhow::{ensure, Result};

use otoge::maimai::models::{DataStore, Song, SongFromAPI};
use otoge::maimai::{get_all_intl_categories, get_all_jp_categories};
use otoge::shared::traits::Otoge;

use super::FetchTask;

trait Maimai {
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

pub struct MaimaiJP;
impl Maimai for MaimaiJP {}

impl Otoge for MaimaiJP {
    type DataStore = DataStore;
    type Song = Song;

    fn name() -> &'static str {
        "maimai_jp"
    }
}

impl FetchTask<Self> for MaimaiJP {
    type ApiSong = SongFromAPI;

    fn api_url() -> &'static str {
        "https://maimai.sega.jp/data/maimai_songs.json"
    }

    fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        Self::impl_verify_categories(data_store)
    }

    fn new_data_store(songs: Vec<<Self as Otoge>::Song>) -> <Self as Otoge>::DataStore {
        DataStore::new(Self::name(), songs, get_all_jp_categories())
    }
}

pub struct MaimaiIntl;
impl Maimai for MaimaiIntl {}

impl Otoge for MaimaiIntl {
    type DataStore = DataStore;
    type Song = Song;

    fn name() -> &'static str {
        "maimai_intl"
    }
}

impl FetchTask<Self> for MaimaiIntl {
    type ApiSong = SongFromAPI;

    fn api_url() -> &'static str {
        "https://maimai.sega.com/assets/data/maimai_songs.json"
    }

    fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        Self::impl_verify_categories(data_store)
    }

    fn new_data_store(songs: Vec<Song>) -> DataStore {
        DataStore::new(Self::name(), songs, get_all_intl_categories())
    }
}
