use models::picture::Picture;
use models::tag::Tag;
use models::correspondent::Correspondent;
use chrono::Utc;
use chrono::DateTime;

#[derive(Serialize)]
pub struct Document {
    pub id : i32,
    pub title : String,
    pub from : Option<Correspondent>,
    #[serde(with = "document_date_format")]
    pub date : DateTime<Utc>,
    #[serde(with = "document_date_format")]
    pub added_on : DateTime<Utc>,
    pub num_pages: i64,
    pub sha256sum: String,
    pub image : Picture,
    pub tags : Vec<Tag>
}

mod document_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%b %e, %Y";
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