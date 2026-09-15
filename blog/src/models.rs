use serde::{Deserialize, Serialize};
use num_rational::Rational64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentItem {
    pub slug: String,
    pub title: String,
    pub html: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub slug: String,
    pub title: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions_html: String,
    pub history: Vec<HistoryEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    #[serde(
        serialize_with = "serialize_rational",
        deserialize_with = "deserialize_rational"
    )]
    pub qty: Option<Rational64>,
    pub unit: String,
    pub name: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub date: String,
    pub title: String,
    pub notes_html: String,
}

// Custom serialization for Rational64
fn serialize_rational<S>(qty: &Option<Rational64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    match qty {
        Some(r) => {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(r.numer())?;
            seq.serialize_element(r.denom())?;
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

// Custom deserialization for Rational64
fn deserialize_rational<'de, D>(deserializer: D) -> Result<Option<Rational64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let opt: Option<(i64, i64)> = Option::deserialize(deserializer)?;
    match opt {
        Some((numer, denom)) => {
            if denom == 0 {
                return Err(D::Error::custom("denominator cannot be zero"));
            }
            Ok(Some(Rational64::new(numer, denom)))
        }
        None => Ok(None),
    }
}
