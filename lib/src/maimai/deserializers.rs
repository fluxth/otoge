use chrono::naive::NaiveDate;
use serde::de::{Deserialize, Deserializer};

pub(crate) fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "000000" {
        Ok(None)
    } else {
        const FORMAT: &str = "%y%m%d";
        NaiveDate::parse_from_str(&s, FORMAT)
            .map(Option::Some)
            .map_err(serde::de::Error::custom)
    }
}
