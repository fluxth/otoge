use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::shared::deserializers::{
    all_default_values_as_none, bool_from_string, empty_string_as_none,
};
use crate::shared::traits::DataStore as DataStoreTrait;

use super::get_all_categories;

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
    #[serde(alias = "lev_ult")]
    #[serde(default = "Option::default")]
    ultima: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct WorldsEndInfo {
    #[serde(alias = "we_kanji")]
    kanji: String,
    #[serde(alias = "we_star")]
    star: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct SongFromAPI {
    id: String,
    title: String,
    #[serde(rename(deserialize = "reading"))]
    title_reading: String,
    artist: String,
    image: String,

    #[serde(rename(deserialize = "catname"))]
    category: String,

    #[serde(deserialize_with = "bool_from_string")]
    #[serde(rename(deserialize = "newflag"))]
    is_new: bool,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    levels: Option<LevelMap>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    worlds_end: Option<WorldsEndInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Song {
    id: String,
    title: String,
    title_reading: String,
    artist: String,
    image: String,
    pub category: String,

    is_new: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    levels: Option<LevelMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    worlds_end: Option<WorldsEndInfo>,
}

impl From<SongFromAPI> for Song {
    fn from(other: SongFromAPI) -> Song {
        Self {
            id: other.id,
            title: other.title,
            title_reading: other.title_reading,
            artist: other.artist,
            image: other.image,
            category: other.category,
            is_new: other.is_new,
            levels: other.levels,
            worlds_end: other.worlds_end,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Category {
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
        self.count != other.count
            || !self.songs.iter().eq(other.songs.iter())
            || !self.categories.iter().eq(other.categories.iter())
    }
}
