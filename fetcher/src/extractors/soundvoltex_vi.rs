use std::{borrow::Cow, sync::Arc};

use async_trait::async_trait;
use scraper::{Html, Selector, error::SelectorErrorKind};
use tokio::{sync::Semaphore, task::JoinSet};
use tracing::{Instrument, info, info_span};

use crate::traits::{Extractor, FetchTask};
use otoge::{
    shared::traits::Otoge,
    soundvoltex::models::{Category, LevelMap, Song},
};

pub struct SoundVoltexVIExtractor;

#[async_trait]
impl<G> Extractor<G> for SoundVoltexVIExtractor
where
    G: Otoge + FetchTask<G>,
    G::Song: Send,
    Vec<<G as Otoge>::Song>: FromIterator<Song>,
{
    async fn fetch_songs() -> anyhow::Result<Vec<G::Song>> {
        let selectors = Selectors::init().unwrap();
        let first_page = 1;
        let name = G::name();

        let first_page_content = get_page_content(first_page).await?;
        let page_count = parse_page_count(first_page_content.html_string.as_str(), &selectors)?;
        info!("Got {} total pages", page_count);

        let mut pages = vec![process_page::<G>(
            first_page,
            first_page_content.html_string,
            &selectors,
        )?];

        let fetch_tasks = (first_page + 1..=page_count).map(get_page_content);

        let semaphore = Arc::new(Semaphore::new(10));
        let mut joinset = JoinSet::new();

        for future in fetch_tasks {
            let sem_local = Arc::clone(&semaphore);
            joinset.spawn(async move {
                let _permit = sem_local.acquire_owned().await.unwrap();
                future.instrument(info_span!("fetch_remote", name)).await
            });
        }

        while let Some(join_result) = joinset.join_next().await {
            let fetch_result = join_result?;
            let fetched_page = fetch_result?;
            pages.push(process_page::<G>(
                fetched_page.page_num,
                fetched_page.html_string,
                &selectors,
            )?);
        }

        let mut extracted = vec![];

        pages.sort_by_key(|p| p.page_num);
        for page in pages {
            extracted.extend(page.songs);
        }

        Ok(extracted)
    }
}

#[derive(Debug)]
struct ExtractedPage<G>
where
    G: Otoge,
{
    page_num: usize,
    songs: Vec<G::Song>,
}

pub struct FetchedPage {
    page_num: usize,
    html_string: String,
}

struct Selectors {
    music_entry: Selector,
    page_select_options: Selector,

    //item_link: Selector,
    item_image: Selector,
    item_genre: Selector,
    item_info: Selector,
    item_level: Selector,
}

impl Selectors {
    fn init<'a>() -> Result<Self, SelectorErrorKind<'a>> {
        Ok(Self {
            music_entry: Selector::parse(r#"#music-result > .music"#)?,
            page_select_options: Selector::parse(r#"#music-result > select#search_page > option"#)?,

            //item_link: Selector::parse(r#".cat > .jk > a[href]"#)?,
            item_image: Selector::parse(r#".cat > .jk > a[href] > img"#)?,
            item_genre: Selector::parse(r#".genre"#)?,
            item_info: Selector::parse(r#".cat > .inner > .info > p"#)?,
            item_level: Selector::parse(r#".cat > .inner > .level > p"#)?,
        })
    }
}

async fn get_page_content(page_num: usize) -> anyhow::Result<FetchedPage> {
    info!("Fetching song index, page {}", page_num);
    let url = "https://p.eagate.573.jp/game/sdvx/vi/music/index.html";

    let resp = reqwest::Client::new()
        .post(url)
        .form(&[("page", page_num.to_string().as_str())])
        .send()
        .await?;

    Ok(FetchedPage {
        page_num,
        html_string: resp.text().await?,
    })
}

fn process_page<G>(
    page_num: usize,
    html_string: String,
    selectors: &Selectors,
) -> anyhow::Result<ExtractedPage<G>>
where
    G: Otoge,
    Vec<<G as Otoge>::Song>: FromIterator<Song>,
{
    info!("Parsing page {}", page_num);
    let html = Html::parse_document(html_string.as_str());

    let songs = html
        .select(&selectors.music_entry)
        .map(|music_entry| {
            let genres = music_entry
                .select(&selectors.item_genre)
                .map(|genre_node| {
                    let genre_id = genre_node
                        .value()
                        .attr("class")
                        .unwrap()
                        .split(' ')
                        .find(|s| *s != "genre")
                        .unwrap();

                    let genre = genre_node.text().next().unwrap();

                    Category {
                        id: Cow::Owned(genre_id.to_owned()),
                        name: Cow::Owned(genre.to_owned()),
                    }
                })
                .collect();

            let mut levels = LevelMap::default();
            for level in music_entry.select(&selectors.item_level) {
                let level_id = level.value().attr("class").unwrap();
                let level_value = level.text().next().unwrap().to_owned();

                match level_id {
                    "nov" => levels.novice = Some(level_value),
                    "adv" => levels.advanced = Some(level_value),
                    "exh" => levels.exhaust = Some(level_value),
                    "mxm" => levels.maximum = Some(level_value),
                    "inf" => levels.infinite = Some(level_value),
                    "grv" => levels.gravity = Some(level_value),
                    "hvn" => levels.heavenly = Some(level_value),
                    "vvd" => levels.vivid = Some(level_value),
                    "xcd" => levels.exceed = Some(level_value),
                    _ => panic!("Unknown level type: {}", level_id),
                }
            }

            let mut info_nodes = music_entry.select(&selectors.item_info);
            let title = info_nodes
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .join(" ");
            let artist = info_nodes
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .join(" ");

            // FIXME: id changes regularly
            //let id = music_entry
            //    .select(&selectors.item_link)
            //    .next()
            //    .unwrap()
            //    .value()
            //    .attr("href")
            //    .unwrap()
            //    .to_owned()
            //    .replacen("/game/sdvx/vi/music/detail.html?music_id=", "", 1);
            //assert!(!id.contains('/'));

            let image = music_entry
                .select(&selectors.item_image)
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap()
                .to_owned();

            Song {
                image,
                title,
                artist,
                categories: genres,
                levels,
            }
        })
        .collect();

    Ok(ExtractedPage::<G> { page_num, songs })
}

fn parse_page_count(html_str: &str, selectors: &Selectors) -> anyhow::Result<usize> {
    let html = Html::parse_document(html_str);
    let mut max_page = 0;

    for option_element in html.select(&selectors.page_select_options) {
        let page_num: usize = option_element
            .value()
            .attr("value")
            .unwrap_or("0")
            .parse()?;
        if page_num > max_page {
            max_page = page_num;
        }
    }

    Ok(max_page)
}
