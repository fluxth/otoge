use serde::de::{Deserialize, Deserializer, Error, IntoDeserializer};

pub fn bool_from_binary_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(Error::unknown_variant(s, &["1", "0"])),
    }
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

pub fn all_default_values_as_none<'de, D, V>(de: D) -> Result<Option<V>, D::Error>
where
    D: Deserializer<'de>,
    V: Default + Deserialize<'de> + PartialEq,
{
    let values = V::deserialize(de)?;
    if values == V::default() {
        // All fields are Default, return None instead of Some(LevelMap)
        return Ok(None);
    }

    Ok(Some(values))
}
