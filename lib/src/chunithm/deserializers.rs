use super::models::{LevelMap, WorldsEndInfo};

use serde::de::{Deserialize, Deserializer};

pub fn empty_worlds_end_as_none<'de, D>(de: D) -> Result<Option<WorldsEndInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    let worlds_end = WorldsEndInfo::deserialize(de)?;
    if worlds_end == WorldsEndInfo::default() {
        // All WorldsEndInfo fields are None (Default), return None instead of Some(WorldsEndInfo)
        return Ok(None);
    }

    Ok(Some(worlds_end))
}

pub fn empty_levels_as_none<'de, D>(de: D) -> Result<Option<LevelMap>, D::Error>
where
    D: Deserializer<'de>,
{
    let levels = LevelMap::deserialize(de)?;
    if levels == LevelMap::default() {
        // All LevelMap fields are None (Default), return None instead of Some(LevelMap)
        return Ok(None);
    }

    Ok(Some(levels))
}
