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
    basic: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_adv")]
    #[serde(default = "Option::default")]
    advanced: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_exp")]
    #[serde(default = "Option::default")]
    expert: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_mas")]
    #[serde(default = "Option::default")]
    master: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "lev_lnt")]
    #[serde(default = "Option::default")]
    lunatic: Option<String>,
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
    pub name: Cow<'static, str>,
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
#[serde(deny_unknown_fields)]
pub struct SongFromAPI {
    id: String,
    title: String,
    #[serde(rename(deserialize = "title_sort"))]
    title_reading: String,
    artist: String,
    #[serde(deserialize_with = "deserialize_date")]
    date: NaiveDate,
    #[serde(rename(deserialize = "new"))]
    #[serde(deserialize_with = "bool_from_string")]
    is_new: bool,
    #[serde(deserialize_with = "bool_from_string")]
    #[serde(rename(deserialize = "lunatic"))]
    is_lunatic: bool,
    #[serde(deserialize_with = "bool_from_string")]
    #[serde(rename(deserialize = "bonus"))]
    is_bonus_track: bool,
    #[serde(rename(deserialize = "image_url"))]
    image: String,
    #[serde(rename(deserialize = "copyright1"))]
    #[serde(deserialize_with = "dash_string_as_none")]
    copyright: Option<String>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    chapter: Option<Chapter>,

    #[serde(flatten)]
    category: CategoryInSong,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    levels: Option<LevelMap>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    character: Option<Character>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Song {
    id: String,
    title: String,
    title_reading: String,
    artist: String,
    date: NaiveDate,
    image: String,
    is_new: bool,
    is_lunatic: bool,
    is_bonus_track: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    chapter: Option<Chapter>,
    pub category: CategoryInSong,
    #[serde(skip_serializing_if = "Option::is_none")]
    levels: Option<LevelMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    character: Option<Character>,
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
    name: Cow<'static, str>,
    count: usize,
    last_updated: DateTime<Utc>,
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
