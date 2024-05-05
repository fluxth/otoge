use std::borrow::Cow;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::get_all_categories;
use crate::shared::deserializers::{all_default_values_as_none, empty_string_as_none};
use crate::shared::traits::DataStore as DataStoreTrait;

#[derive(Serialize)]
pub struct APIInput {
    service_kind: &'static str,
}

impl Default for APIInput {
    fn default() -> Self {
        Self {
            service_kind: "music_list",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct LevelMap {
    #[serde(deserialize_with = "all_default_values_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    easy: Option<i32>,
    #[serde(deserialize_with = "all_default_values_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    normal: Option<i32>,
    #[serde(deserialize_with = "all_default_values_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    hard: Option<i32>,
    #[serde(deserialize_with = "all_default_values_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    influence: Option<i32>,
    #[serde(deserialize_with = "all_default_values_as_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    polar: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct SongFromAPI {
    music_id: String,
    genre: u32,
    name: String,
    composer: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    license: Option<String>,
    #[serde(flatten)]
    levels: LevelMap,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Song {
    id: String,
    title: String,
    artist: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    levels: LevelMap,
    categories: Vec<Category>,
}

impl From<SongFromAPI> for Song {
    fn from(value: SongFromAPI) -> Self {
        let bitflags = value.genre;
        let mut categories = vec![];

        for (index, category) in get_all_categories().into_iter().enumerate() {
            if (bitflags >> index) & 1 > 0 {
                categories.push(category)
            }
        }

        Self {
            id: value.music_id,
            title: value.name,
            artist: value.composer,
            license: value.license,
            levels: value.levels,
            categories,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Category {
    pub bitflag: u32,
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
