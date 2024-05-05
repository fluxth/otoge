use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize};

use crate::traits::{Extractor, FetchTask};
use otoge::{polarischord::models::APIInput, shared::traits::Otoge};

#[derive(Deserialize, Debug)]
struct APIResponse<T> {
    data: APIResponseData<T>,
}

#[derive(Deserialize, Debug)]
struct APIResponseData<T> {
    musiclist: APIResponseMusicList<T>,
}

#[derive(Deserialize, Debug)]
struct APIResponseMusicList<T> {
    music: Vec<T>,
}

pub struct PolarisChordExtractor;

#[async_trait]
impl<G> Extractor<G> for PolarisChordExtractor
where
    G: Otoge + FetchTask<G>,
    G::Song: std::convert::From<G::ApiSong>,
    G::ApiSong: DeserializeOwned + std::fmt::Debug,
{
    async fn fetch_songs() -> anyhow::Result<Vec<G::Song>> {
        let input_data = APIInput::default();
        let url = G::api_url();

        let client = reqwest::Client::new();
        let resp = client.post(url).form(&input_data).send().await?;
        let data = resp.json::<APIResponse<G::ApiSong>>().await?;

        Ok(data
            .data
            .musiclist
            .music
            .into_iter()
            .map(|song| song.into())
            .collect())
    }
}
