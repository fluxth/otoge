use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::shared::deserializers::{
    all_default_values_as_none, bool_from_option_string, empty_string_as_none,
};
use crate::shared::traits::DataStore as DataStoreTrait;

use super::deserializers::deserialize_date;

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
    #[serde(alias = "lev_remas")]
    #[serde(default = "Option::default")]
    remaster: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct DXLevelMap {
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "dx_lev_bas")]
    #[serde(default = "Option::default")]
    basic: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "dx_lev_adv")]
    #[serde(default = "Option::default")]
    advanced: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "dx_lev_exp")]
    #[serde(default = "Option::default")]
    expert: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "dx_lev_mas")]
    #[serde(default = "Option::default")]
    master: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    // FIXME: Be more explicit when deserializing API
    #[serde(alias = "dx_lev_remas")]
    #[serde(default = "Option::default")]
    remaster: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct Utage {
    #[serde(rename(deserialize = "lev_utage"))]
    level: String,
    kanji: String,
    comment: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct SongFromAPI {
    #[serde(rename(deserialize = "sort"))]
    id: String,
    title: String,
    #[serde(rename(deserialize = "title_kana"))]
    title_reading: String,
    artist: String,
    #[serde(rename(deserialize = "image_url"))]
    image: String,
    #[serde(rename(deserialize = "catcode"))]
    category: String,
    #[serde(deserialize_with = "deserialize_date")]
    release: Option<NaiveDate>,
    version: String,
    #[serde(rename(deserialize = "date"))]
    #[serde(deserialize_with = "bool_from_option_string")]
    #[serde(default = "bool::default")]
    is_new: bool,
    #[serde(rename(deserialize = "key"))]
    #[serde(deserialize_with = "bool_from_option_string")]
    #[serde(default = "bool::default")]
    is_locked: bool,

    #[serde(default = "Option::default")]
    buddy: Option<String>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    levels: Option<LevelMap>,

    #[serde(flatten)]
    #[serde(deserialize_with = "all_default_values_as_none")]
    dx_levels: Option<DXLevelMap>,

    #[serde(flatten)]
    utage: Option<Utage>,
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
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    release: Option<NaiveDate>,
    is_new: bool,
    is_locked: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    levels: Option<LevelMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dx_levels: Option<DXLevelMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    utage: Option<Utage>,
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
            version: other.version,
            is_new: other.is_new,
            is_locked: other.is_locked,
            release: other.release,
            levels: other.levels,
            dx_levels: other.dx_levels,
            utage: other.utage,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Category {
    pub slug: Cow<'static, str>,
    pub name: Cow<'static, str>,
}

trait Region {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStore {
    name: Cow<'static, str>,
    count: usize,
    last_updated: DateTime<Utc>,
    pub songs: Vec<Song>,
    pub categories: Vec<Category>,
}

impl DataStore {
    pub fn new(name: &'static str, songs: Vec<Song>, categories: Vec<Category>) -> Self {
        Self {
            name: Cow::Borrowed(name),
            count: songs.len(),
            songs,
            last_updated: Utc::now(),
            categories,
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
