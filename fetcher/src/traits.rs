use anyhow::Result;

use async_trait::async_trait;
use otoge::shared::traits::Otoge;

pub trait FetchTask<G>
where
    G: Otoge,
{
    type ApiSong;
    type Extractor;

    fn api_url() -> &'static str;

    fn new_data_store(songs: Vec<G::Song>) -> G::DataStore;
    fn verify_categories(_data_store: &G::DataStore) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub trait Extractor<G>
where
    G: Otoge + FetchTask<G>,
{
    async fn fetch_songs() -> Result<Vec<G::Song>>;
}
