use std::collections::HashSet;

use anyhow::{ensure, Result};
use async_trait::async_trait;

use crate::extractors::soundvoltex_vi::SoundVoltexVIExtractor;
use crate::traits::FetchTask;
use otoge::shared::traits::Otoge;
use otoge::soundvoltex::models::{DataStore, Song};
use otoge::soundvoltex::SoundVoltex;

#[async_trait]
impl FetchTask<Self> for SoundVoltex {
    type ApiSong = Song;
    type Extractor = SoundVoltexVIExtractor;

    fn api_url() -> &'static str {
        "https://p.eagate.573.jp/game/sdvx/vi/music/index.html"
    }

    fn new_data_store(songs: Vec<<Self as Otoge>::Song>) -> <Self as Otoge>::DataStore {
        DataStore::new(Self::name(), songs)
    }

    async fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        let categories = &data_store.categories;
        let songs = &data_store.songs;

        let all_categories: HashSet<(&str, &str)> = categories
            .iter()
            .map(|cat| (cat.id.as_ref(), cat.name.as_ref()))
            .collect();

        let mut song_categories = HashSet::<(&str, &str)>::new();
        for song in songs {
            for category in &song.categories {
                song_categories.insert((category.id.as_ref(), category.name.as_ref()));
            }
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
