use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date = DateTime::parse_from_rfc3339(&s);

    match date {
        Ok(date) => Ok(DateTime::<Utc>::from_naive_utc_and_offset(date.naive_utc(), Utc)),
        Err(_) => Err(serde::de::Error::custom("Invalid date format")),
    }
}
