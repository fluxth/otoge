mod deserializers;

use crate::deserializers::{bool_from_binary_string, empty_string_as_none};
use deserializers::{empty_levels_as_none, empty_we_as_none};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    #[serde(alias = "lev_ult")]
    #[serde(default = "Option::default")]
    pub ultima: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(dead_code)]
pub struct WorldsEndInfo {
    #[serde(alias = "we_kanji")]
    pub kanji: String,
    #[serde(alias = "we_star")]
    pub star: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SongFromAPI {
    pub id: String,
    pub title: String,
    #[serde(rename(deserialize = "reading"))]
    pub title_reading: String,
    pub artist: String,
    pub image: String,

    #[serde(rename(deserialize = "catname"))]
    pub category: String,

    #[serde(deserialize_with = "bool_from_binary_string")]
    #[serde(rename(deserialize = "newflag"))]
    pub is_new: bool,

    #[serde(flatten)]
    #[serde(deserialize_with = "empty_levels_as_none")]
    pub levels: Option<LevelMap>,

    #[serde(flatten)]
    #[serde(deserialize_with = "empty_we_as_none")]
    pub worlds_end: Option<WorldsEndInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(dead_code)]
//#[serde(from = "SongFromAPI")]
pub struct Song {
    id: String,
    title: String,
    title_reading: String,
    artist: String,
    image: String,
    category: String,

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStore {
    pub name: String,
    pub songs: Vec<Song>,
    pub count: usize,
    pub last_updated: DateTime<Utc>,
}

impl DataStore {
    pub fn new(name: &str, songs: Vec<Song>) -> Self {
        Self {
            name: name.to_owned(),
            count: songs.len(),
            songs,
            last_updated: Utc::now(),
        }
    }

    pub fn data_differs(&self, other: &Self) -> bool {
        if self.count != other.count {
            true
        } else if !self.songs.iter().eq(other.songs.iter()) {
            true
        } else {
            false
        }
    }
}
