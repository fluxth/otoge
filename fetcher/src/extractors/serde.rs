use async_trait::async_trait;

use otoge::shared::traits::Otoge;
use serde::de::DeserializeOwned;

use crate::traits::{Extractor, FetchTask};

pub struct SerdeExtractor;

#[async_trait]
impl<G> Extractor<G> for SerdeExtractor
where
    G: Otoge + FetchTask<G>,
    G::Song: std::convert::From<G::ApiSong>,
    G::ApiSong: DeserializeOwned,
{
    async fn fetch_songs() -> anyhow::Result<Vec<G::Song>> {
        let resp = reqwest::get(G::api_url()).await?;
        let data = resp.json::<Vec<G::ApiSong>>().await?;

        Ok(data.into_iter().map(|song| song.into()).collect())
    }
}
