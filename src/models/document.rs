use models::picture::Picture;
use models::tag::Tag;
use chrono::Utc;
use chrono::DateTime;

#[derive(Serialize)]
pub struct Document {
    pub title : String,
    pub from : String,
    #[serde(with = "document_date_format")]
    pub date : DateTime<Utc>,
    pub image : Picture,
    pub tags : Vec<Tag>
}

mod document_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}