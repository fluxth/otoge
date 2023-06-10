use std::borrow::Cow;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::get_all_categories;
use crate::shared::traits::DataStore as DataStoreTrait;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct LevelMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub novice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exhaust: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heavenly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vivid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Song {
    pub id: String,
    pub image: String,
    pub title: String,
    pub artist: String,
    pub categories: Vec<Category>,
    pub levels: LevelMap,
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
