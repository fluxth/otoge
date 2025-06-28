use serde::de::{Deserialize, Deserializer, Error, IntoDeserializer};

pub(crate) fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "1" | "NEW" => Ok(true),
        "0" | "" => Ok(false),
        _ => Err(Error::unknown_variant(s, &["NEW", "1", "0", ""])),
    }
}

pub(crate) fn bool_from_option_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;

    match s {
        Some("NEW" | "○") => Ok(true),
        None => Ok(false),
        _ => Err(Error::unknown_variant(
            &format!("{s:?}"),
            &["Some(\"NEW\")", "Some(\"○\")", "None"],
        )),
    }
}

pub(crate) fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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

pub(crate) fn all_default_values_as_none<'de, D, V>(de: D) -> Result<Option<V>, D::Error>
where
    D: Deserializer<'de>,
    V: Default + Deserialize<'de> + PartialEq,
{
    let values = V::deserialize(de)?;
    if values == V::default() {
        // All fields are Default, return None instead of Some(V)
        return Ok(None);
    }

    Ok(Some(values))
}
