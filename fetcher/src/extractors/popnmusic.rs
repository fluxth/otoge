use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use scraper::Html;
use scraper::Selector;
use scraper::error::SelectorErrorKind;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tracing::{Instrument, info, info_span};

use crate::traits::{Extractor, FetchTask};
use otoge::popnmusic::models::{Category, LevelMap, Song};
use otoge::popnmusic::{get_all_bemani, get_all_categories, get_all_versions};
use otoge::shared::traits::Otoge;

pub struct PopNMusicExtractor;

#[async_trait]
impl<G> Extractor<G> for PopNMusicExtractor
where
    G: Otoge + FetchTask<G>,
    G::Song: Send,
    Vec<<G as Otoge>::Song>: FromIterator<Song>,
{
    async fn fetch_songs(client: &reqwest::Client) -> anyhow::Result<Vec<G::Song>> {
        let selectors = Arc::new(
            Selectors::init()
                .map_err(|e| anyhow::anyhow!("Failed to initialize CSS selectors: {e}"))?,
        );

        let url = G::api_url();
        let semaphore = Arc::new(Semaphore::new(10));

        info!("Fetching all songs");

        let all_songs_filter = PageFilter::no_filter();

        let all_songs_first_page = {
            let _permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            get_page_content(client, url, 0, &all_songs_filter).await?
        };

        verify_filter_options(&all_songs_first_page.html_string, &selectors)?;

        let all_songs = fetch_pages_for_filter(
            client.clone(),
            url,
            all_songs_filter,
            Arc::clone(&selectors),
            Arc::clone(&semaphore),
            Some(all_songs_first_page),
        )
        .instrument(info_span!("fetch_all"))
        .await?;

        info!("Fetched {} songs", all_songs.len());

        info!("Fetching version, bemani, and recommendation category song lists");

        let all_bemani = get_all_bemani();
        let bemani_order: HashMap<String, usize> = all_bemani
            .iter()
            .enumerate()
            .map(|(index, bemani)| (bemani.id.as_ref().to_owned(), index))
            .collect();

        let all_categories = get_all_categories();
        let category_order: HashMap<String, usize> = all_categories
            .iter()
            .enumerate()
            .map(|(index, category)| (category.id.as_ref().to_owned(), index))
            .collect();

        let mut joinset: JoinSet<anyhow::Result<FilterResult>> = JoinSet::new();

        for version in get_all_versions() {
            let version_id = version.id.as_ref().to_owned();
            let client = client.clone();
            let selectors = Arc::clone(&selectors);
            let semaphore = Arc::clone(&semaphore);
            let span = info_span!("fetch_version", id = version_id);

            joinset.spawn(
                async move {
                    let filter = PageFilter {
                        version: version_id.into(),
                        ..PageFilter::no_filter()
                    };

                    let songs =
                        fetch_pages_for_filter(client, url, filter, selectors, semaphore, None)
                            .await?;

                    Ok(FilterResult::Version(version, songs))
                }
                .instrument(span),
            );
        }

        for bemani in all_bemani {
            let bemani_id = bemani.id.as_ref().to_owned();
            let client = client.clone();
            let selectors = Arc::clone(&selectors);
            let semaphore = Arc::clone(&semaphore);
            let span = info_span!("fetch_bemani", id = bemani_id);

            joinset.spawn(
                async move {
                    let filter = PageFilter {
                        bemani: bemani_id.into(),
                        ..PageFilter::no_filter()
                    };

                    let songs =
                        fetch_pages_for_filter(client, url, filter, selectors, semaphore, None)
                            .await?;

                    Ok(FilterResult::Bemani(bemani, songs))
                }
                .instrument(span),
            );
        }

        for category in all_categories {
            let category_id = category.id.as_ref().to_owned();
            let client = client.clone();
            let selectors = Arc::clone(&selectors);
            let semaphore = Arc::clone(&semaphore);
            let span = info_span!("fetch_recommendations", id = category_id);

            joinset.spawn(
                async move {
                    let filter = PageFilter {
                        category: category_id.into(),
                        ..PageFilter::no_filter()
                    };

                    let songs =
                        fetch_pages_for_filter(client, url, filter, selectors, semaphore, None)
                            .await?;

                    Ok(FilterResult::RecommendationCategory(category, songs))
                }
                .instrument(span),
            );
        }

        let mut version_map: HashMap<SongKey, Category> = HashMap::new();
        let mut bemani_map: HashMap<SongKey, Vec<Category>> = HashMap::new();
        let mut category_map: HashMap<SongKey, Vec<Category>> = HashMap::new();

        while let Some(result) = joinset.join_next().await {
            match result?? {
                FilterResult::Version(version, songs) => {
                    for parsed in songs {
                        let key = SongKey::from_parsed(&parsed);

                        anyhow::ensure!(
                            !version_map.contains_key(&key),
                            "Song '{}' found in multiple versions",
                            parsed.title
                        );

                        version_map.insert(key, version.clone());
                    }
                }
                FilterResult::Bemani(bemani, songs) => {
                    for parsed in songs {
                        let key = SongKey::from_parsed(&parsed);
                        bemani_map.entry(key).or_default().push(bemani.clone());
                    }
                }
                FilterResult::RecommendationCategory(category, songs) => {
                    for parsed in songs {
                        let key = SongKey::from_parsed(&parsed);
                        category_map.entry(key).or_default().push(category.clone());
                    }
                }
            }
        }

        for entries in bemani_map.values_mut() {
            entries.sort_by_key(|bemani| bemani_order[bemani.id.as_ref()]);
        }

        for entries in category_map.values_mut() {
            entries.sort_by_key(|category| category_order[category.id.as_ref()]);
        }

        let mut songs = Vec::with_capacity(all_songs.len());

        for parsed in all_songs {
            let key = SongKey::from_parsed(&parsed);

            songs.push(Song {
                image: parsed.image,
                genre: parsed.genre,
                title: parsed.title,
                artist: parsed.artist,
                levels: parsed.levels,
                version: version_map.get(&key).cloned(),
                bemani: bemani_map.get(&key).cloned().unwrap_or_default(),
                recommendation_categories: category_map.get(&key).cloned().unwrap_or_default(),
            });
        }

        info!("Assembled {} songs", songs.len());

        Ok(songs.into_iter().collect())
    }
}

#[derive(Hash, PartialEq, Eq)]
struct SongKey {
    title: String,
    artist: String,
    genre: String,
    image: String,
}

impl SongKey {
    fn from_parsed(parsed: &ParsedSong) -> Self {
        Self {
            title: parsed.title.clone(),
            artist: parsed.artist.clone(),
            genre: parsed.genre.clone(),
            image: parsed.image.clone(),
        }
    }
}

enum FilterResult {
    Version(Category, Vec<ParsedSong>),
    Bemani(Category, Vec<ParsedSong>),
    RecommendationCategory(Category, Vec<ParsedSong>),
}

struct ParsedSong {
    image: String,
    genre: String,
    title: String,
    artist: String,
    levels: LevelMap,
}

struct PageFilter {
    version: Cow<'static, str>,
    bemani: Cow<'static, str>,
    category: Cow<'static, str>,
}

impl PageFilter {
    // Sentinel values for "no filter":
    // version="-1" means all versions, bemani/category="0" means all.
    fn no_filter() -> Self {
        Self {
            version: "-1".into(),
            bemani: "0".into(),
            category: "0".into(),
        }
    }
}

struct FetchedPage {
    page_num: usize,
    html_string: String,
}

struct Selectors {
    list_items: Selector,
    page_select_options: Selector,
    version_options: Selector,
    bemani_options: Selector,
    category_options: Selector,
    item_image: Selector,
    item_info: Selector,
    item_level_span: Selector,
}

impl Selectors {
    fn init<'a>() -> Result<Self, SelectorErrorKind<'a>> {
        Ok(Self {
            list_items: Selector::parse(r#"ul.mu_list_table:not(.mu_head) > li"#)?,
            page_select_options: Selector::parse(r#"select#s_page > option"#)?,
            version_options: Selector::parse(r#"select#s_version > option"#)?,
            bemani_options: Selector::parse(r#"select#s_bemani > option"#)?,
            category_options: Selector::parse(r#"select#s_cate > option"#)?,
            item_image: Selector::parse(r#"img"#)?,
            item_info: Selector::parse(r#"p"#)?,
            item_level_span: Selector::parse(r#"span"#)?,
        })
    }
}

fn verify_filter_options(html: &str, selectors: &Selectors) -> anyhow::Result<()> {
    use std::collections::HashSet;

    let all_versions = get_all_versions();
    let known_version_ids: HashSet<&str> = all_versions.iter().map(|v| v.id.as_ref()).collect();

    let all_bemani = get_all_bemani();
    let known_bemani_ids: HashSet<&str> = all_bemani.iter().map(|b| b.id.as_ref()).collect();

    let all_categories = get_all_categories();
    let known_category_ids: HashSet<&str> = all_categories.iter().map(|c| c.id.as_ref()).collect();

    let document = Html::parse_document(html);

    let unknown_versions: Vec<&str> = document
        .select(&selectors.version_options)
        .filter_map(|option| option.value().attr("value"))
        .filter(|id| *id != "-1" && !known_version_ids.contains(id)) // "-1" = all versions sentinel
        .collect();

    anyhow::ensure!(
        unknown_versions.is_empty(),
        "Unknown version ids on site: {:?}",
        unknown_versions
    );

    let unknown_bemani: Vec<&str> = document
        .select(&selectors.bemani_options)
        .filter_map(|option| option.value().attr("value"))
        .filter(|id| *id != "0" && !known_bemani_ids.contains(id)) // "0" = no filter sentinel
        .collect();

    anyhow::ensure!(
        unknown_bemani.is_empty(),
        "Unknown bemani ids on site: {:?}",
        unknown_bemani
    );

    let unknown_categories: Vec<&str> = document
        .select(&selectors.category_options)
        .filter_map(|option| option.value().attr("value"))
        .filter(|id| *id != "0" && !known_category_ids.contains(id))
        .collect();

    anyhow::ensure!(
        unknown_categories.is_empty(),
        "Unknown recommendation category ids on site: {:?}",
        unknown_categories
    );

    Ok(())
}

async fn fetch_pages_for_filter(
    client: reqwest::Client,
    base_url: &'static str,
    filter: PageFilter,
    selectors: Arc<Selectors>,
    semaphore: Arc<Semaphore>,
    prefetched_first_page: Option<FetchedPage>,
) -> anyhow::Result<Vec<ParsedSong>> {
    let first_page = match prefetched_first_page {
        Some(page) => page,
        None => {
            let _permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            get_page_content(&client, base_url, 0, &filter).await?
        }
    };

    let last_page = parse_for_last_page(&first_page.html_string, &selectors)?;
    info!("Got {} total pages", last_page + 1);

    let mut pages: Vec<(usize, Vec<ParsedSong>)> = vec![(
        0,
        parse_songs_from_page(&first_page.html_string, &selectors)?,
    )];

    let mut joinset = JoinSet::new();

    for page_num in 1..=last_page {
        let sem = Arc::clone(&semaphore);
        let client = client.clone();
        let version = filter.version.clone();
        let bemani = filter.bemani.clone();
        let category = filter.category.clone();

        let span = tracing::Span::current();

        joinset.spawn(
            async move {
                let _permit = sem.acquire_owned().await.unwrap();

                let filter = PageFilter {
                    version,
                    bemani,
                    category,
                };

                get_page_content(&client, base_url, page_num, &filter).await
            }
            .instrument(span),
        );
    }

    while let Some(result) = joinset.join_next().await {
        let fetched = result??;
        pages.push((
            fetched.page_num,
            parse_songs_from_page(&fetched.html_string, &selectors)?,
        ));
    }

    pages.sort_by_key(|(page_num, _)| *page_num);

    Ok(pages.into_iter().flat_map(|(_, songs)| songs).collect())
}

async fn get_page_content(
    client: &reqwest::Client,
    base_url: &str,
    page_num: usize,
    filter: &PageFilter,
) -> anyhow::Result<FetchedPage> {
    info!("Fetching page {}", page_num + 1);

    let resp = client
        .get(base_url)
        .query(&[
            ("page", page_num.to_string().as_str()),
            ("version", filter.version.as_ref()),
            ("lv", "0"),
            ("bemani", filter.bemani.as_ref()),
            ("category", filter.category.as_ref()),
            ("keyword", ""),
            ("sort", "music"),
            ("sort_type", "none"),
        ])
        .send()
        .await?
        .error_for_status()?;

    Ok(FetchedPage {
        page_num,
        html_string: resp.text().await?,
    })
}

fn parse_for_last_page(html_str: &str, selectors: &Selectors) -> anyhow::Result<usize> {
    let html = Html::parse_document(html_str);
    let mut max_page = 0;

    for option in html.select(&selectors.page_select_options) {
        let value: usize = option.value().attr("value").unwrap_or("0").parse()?;
        if value > max_page {
            max_page = value;
        }
    }

    Ok(max_page)
}

fn parse_songs_from_page(html_str: &str, selectors: &Selectors) -> anyhow::Result<Vec<ParsedSong>> {
    let html = Html::parse_document(html_str);

    let items: Vec<_> = html.select(&selectors.list_items).collect();

    items
        .chunks(3)
        .map(|chunk| {
            anyhow::ensure!(
                chunk.len() == 3,
                "Expected 3 list items per song entry, got {}",
                chunk.len()
            );

            let image = chunk[0]
                .select(&selectors.item_image)
                .next()
                .ok_or_else(|| anyhow::anyhow!("Song item missing img element"))?
                .value()
                .attr("src")
                .ok_or_else(|| anyhow::anyhow!("Song img missing src attribute"))?
                .to_owned();

            let info_nodes: Vec<_> = chunk[1].select(&selectors.item_info).collect();
            anyhow::ensure!(
                info_nodes.len() == 3,
                "Expected 3 info nodes, got {}",
                info_nodes.len()
            );
            let genre = info_nodes[0].text().collect::<Vec<_>>().join("");
            let title = info_nodes[1].text().collect::<Vec<_>>().join("");
            let artist = info_nodes[2].text().collect::<Vec<_>>().join("");

            let mut levels = LevelMap::default();
            for level_p in chunk[2].select(&selectors.item_info) {
                let level_value = level_p.value().attr("data-d").unwrap_or("-");
                if level_value == "-" {
                    continue;
                }

                let level_name = level_p
                    .select(&selectors.item_level_span)
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Level item missing span element"))?
                    .text()
                    .collect::<Vec<_>>()
                    .join("");

                let value = Some(level_value.to_owned());
                match level_name.as_str() {
                    "LIGHT" => levels.light = value,
                    "NORMAL" => levels.normal = value,
                    "HYPER" => levels.hyper = value,
                    "EX" => levels.ex = value,
                    _ => anyhow::bail!("Unknown level type '{level_name}'"),
                }
            }

            Ok(ParsedSong {
                image,
                genre,
                title,
                artist,
                levels,
            })
        })
        .collect()
}
