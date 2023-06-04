use super::models::{LevelMap, WorldsEndInfo};

use serde::de::{Deserialize, Deserializer};

pub fn empty_we_as_none<'de, D>(de: D) -> Result<Option<WorldsEndInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    let we = WorldsEndInfo::deserialize(de)?;
    match (we.kanji.as_str(), we.star.as_str()) {
        ("", "") => Ok(None),
        _ => Ok(Some(we)),
    }
}

pub fn empty_levels_as_none<'de, D>(de: D) -> Result<Option<LevelMap>, D::Error>
where
    D: Deserializer<'de>,
{
    let levels = LevelMap::deserialize(de)?;
    if levels.basic.is_none()
        && levels.advanced.is_none()
        && levels.expert.is_none()
        && levels.master.is_none()
        && levels.ultima.is_none()
    {
        return Ok(None);
    }

    Ok(Some(levels))
}
