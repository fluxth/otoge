use std::collections::HashSet;

use anyhow::{Result, ensure};
use async_trait::async_trait;

use crate::extractors::popnmusic::PopNMusicExtractor;
use crate::traits::FetchTask;
use otoge::popnmusic::PopNMusic;
use otoge::popnmusic::models::{DataStore, Song};
use otoge::shared::traits::Otoge;

#[async_trait]
impl FetchTask<Self> for PopNMusic {
    type ApiSong = Song;
    type Extractor = PopNMusicExtractor;

    fn api_url() -> &'static str {
        "https://p.eagate.573.jp/game/popn/popn29/music/list.html"
    }

    fn new_data_store(songs: Vec<<Self as Otoge>::Song>) -> <Self as Otoge>::DataStore {
        DataStore::new(Self::name(), songs)
    }

    async fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        let versions = &data_store.versions;
        let bemani_games = &data_store.bemani;
        let categories = &data_store.categories;
        let songs = &data_store.songs;

        let valid_version_ids: HashSet<&str> = versions.iter().map(|v| v.id.as_ref()).collect();
        let valid_bemani_ids: HashSet<&str> = bemani_games.iter().map(|b| b.id.as_ref()).collect();
        let valid_category_ids: HashSet<&str> = categories.iter().map(|c| c.id.as_ref()).collect();

        for song in songs {
            if let Some(version) = &song.version {
                ensure!(
                    valid_version_ids.contains(version.id.as_ref()),
                    "Unknown version id '{}' on song '{}'",
                    version.id,
                    song.title
                );
            }

            for bemani in &song.bemani {
                ensure!(
                    valid_bemani_ids.contains(bemani.id.as_ref()),
                    "Unknown BEMANI id '{}' on song '{}'",
                    bemani.id,
                    song.title
                );
            }

            for cat in &song.recommendation_categories {
                ensure!(
                    valid_category_ids.contains(cat.id.as_ref()),
                    "Unknown category id '{}' on song '{}'",
                    cat.id,
                    song.title
                );
            }
        }

        Ok(())
    }
}
