use std::borrow::Cow;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{get_all_bemani, get_all_categories, get_all_versions};
use crate::shared::traits::DataStore as DataStoreTrait;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Category {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Hash)]
pub struct LevelMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Song {
    pub image: String,
    pub genre: String,
    pub title: String,
    pub artist: String,
    pub levels: LevelMap,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Category>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bemani: Vec<Category>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendation_categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStore {
    name: Cow<'static, str>,
    count: usize,
    last_updated: DateTime<Utc>,
    pub songs: Vec<Song>,
    pub versions: Vec<Category>,
    pub bemani: Vec<Category>,
    pub categories: Vec<Category>,
}

impl DataStore {
    pub fn new(name: &'static str, songs: Vec<Song>) -> Self {
        Self {
            name: Cow::Borrowed(name),
            count: songs.len(),
            songs,
            last_updated: Utc::now(),
            versions: get_all_versions(),
            bemani: get_all_bemani(),
            categories: get_all_categories(),
        }
    }
}

impl DataStoreTrait for DataStore {
    fn data_differs(&self, other: &Self) -> bool {
        self.count != other.count
            || !self.songs.iter().eq(other.songs.iter())
            || !self.versions.iter().eq(other.versions.iter())
            || !self.bemani.iter().eq(other.bemani.iter())
            || !self.categories.iter().eq(other.categories.iter())
    }
}
