use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::deserializers::{dash_string_as_none, deserialize_date};
use super::get_all_categories;
use crate::shared::deserializers::{
    all_default_values_as_none, bool_from_string, empty_string_as_none,
};
use crate::shared::traits::DataStore as DataStoreTrait;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct LevelMap {
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_bas")]
    #[serde(default = "Option::default")]
    pub basic: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_adv")]
    #[serde(default = "Option::default")]
    pub advanced: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_exp")]
    #[serde(default = "Option::default")]
    pub expert: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_mas")]
    #[serde(default = "Option::default")]
    pub master: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_lnt")]
    #[serde(default = "Option::default")]
    pub lunatic: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Character {
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "chara_id")]
    id: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "character")]
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct CategoryInSong {
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "category_id")]
    pub id: Cow<'static, str>,
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "category")]
    name: Cow<'static, str>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Chapter {
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "chap_id")]
    id: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "chapter")]
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SongFromAPI {
    pub id: String,
    pub title: String,
    #[serde(rename(deserialize = "title_sort"))]
    pub title_reading: String,
    pub artist: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    #[serde(rename(deserialize = "new"))]
    #[serde(deserialize_with = "bool_from_string")]
    pub is_new: bool,
    #[serde(deserialize_with = "bool_from_string")]
    #[serde(rename(deserialize = "lunatic"))]
    pub is_lunatic: bool,
    #[serde(deserialize_with = "bool_from_string")]
    #[serde(rename(deserialize = "bonus"))]
    pub is_bonus_track: bool,
    #[serde(rename(deserialize = "image_url"))]
    pub image: String,
    #[serde(rename(deserialize = "copyright1"))]
    #[serde(deserialize_with = "dash_string_as_none")]
    pub copyright: Option<String>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    pub chapter: Option<Chapter>,

    #[serde(flatten)]
    pub category: CategoryInSong,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    pub levels: Option<LevelMap>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    pub character: Option<Character>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub title_reading: String,
    pub artist: String,
    pub date: NaiveDate,
    pub image: String,
    pub is_new: bool,
    pub is_lunatic: bool,
    pub is_bonus_track: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<Chapter>,
    pub category: CategoryInSong,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<LevelMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<Character>,
}

impl From<SongFromAPI> for Song {
    fn from(other: SongFromAPI) -> Song {
        Self {
            id: other.id,
            title: other.title,
            title_reading: other.title_reading,
            artist: other.artist,
            date: other.date,
            image: other.image,
            is_new: other.is_new,
            is_lunatic: other.is_lunatic,
            is_bonus_track: other.is_bonus_track,
            copyright: other.copyright,

            chapter: other.chapter,
            category: other.category,
            levels: other.levels,
            character: other.character,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Category {
    pub id: Cow<'static, str>,
    pub slug: Cow<'static, str>,
    pub name: Cow<'static, str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStore {
    pub name: Cow<'static, str>,
    pub count: usize,
    pub last_updated: DateTime<Utc>,
    pub songs: Vec<Song>,
    pub categories: Vec<Category>,
}

impl DataStore {
    pub fn new(name: &'static str, songs: Vec<Song>) -> Self {
        Self {
            name: Cow::Borrowed(name),
            count: songs.len(),
            songs,
            last_updated: Utc::now(),
            categories: get_all_categories(),
        }
    }
}

impl DataStoreTrait for DataStore {
    fn data_differs(&self, other: &Self) -> bool {
        self.count != other.count || !self.songs.iter().eq(other.songs.iter())
    }
}
