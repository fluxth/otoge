use std::borrow::Cow;

use anyhow::{Context, Result, anyhow, ensure};
use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::extractors::polarischord::PolarisChordExtractor;
use crate::traits::FetchTask;
use otoge::polarischord::PolarisChord;
use otoge::polarischord::models::{Category, DataStore, Song, SongFromAPI};
use otoge::shared::traits::Otoge;

async fn fetch_categories() -> Result<Vec<Category>> {
    let url = "https://p.eagate.573.jp/game/polarischord/pc/music/index.html";

    let resp = reqwest::get(url).await?;
    let html_string = resp.text().await?;

    let html = Html::parse_document(html_string.as_str());
    let selector = Selector::parse(r#"div#search > .category_select > select > option"#)
        .or(Err(anyhow!("selector parse failed")))?;

    let mut categories = vec![];

    for option_element in html.select(&selector) {
        let bitflag = option_element
            .value()
            .attr("data-index")
            .context("no category data-index found")?
            .parse()?;
        let slug = option_element
            .value()
            .attr("value")
            .context("no category value found")?
            .to_string();
        let name = option_element
            .text()
            .map(|t| t.trim().to_string())
            .collect::<Vec<_>>()
            .join("");

        if bitflag == 0 && slug == "all" {
            continue;
        }

        categories.push(Category {
            bitflag,
            slug: Cow::Owned(slug),
            name: Cow::Owned(name),
        })
    }

    Ok(categories)
}

#[async_trait]
impl FetchTask<Self> for PolarisChord {
    type ApiSong = SongFromAPI;
    type Extractor = PolarisChordExtractor;

    fn api_url() -> &'static str {
        "https://p.eagate.573.jp/game/polarischord/pc/json/common_getdata.html"
    }

    fn new_data_store(songs: Vec<Song>) -> DataStore {
        DataStore::new(Self::name(), songs)
    }

    async fn verify_categories(data_store: &<Self as Otoge>::DataStore) -> Result<()> {
        let local_categories = data_store.categories.as_slice();
        let fetched_categories = fetch_categories().await?;

        ensure!(
            local_categories == fetched_categories.as_slice(),
            "Local category definitions differs, {:#?} != {:#?}",
            local_categories,
            fetched_categories,
        );

        Ok(())
    }
}
