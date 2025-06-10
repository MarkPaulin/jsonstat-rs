use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use serde::{Deserialize, Serialize};


type DimensionId = String;

#[derive(Deserialize, Serialize, PartialEq)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2_0,
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::V2_0 => write!(f, "2.0")
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Class {
    Dataset,
    Dimension,
    Collection,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StatValue {
    Array(Vec<f64>),
    Dictionary(HashMap<String, f64>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Status {
    Array(Vec<String>),
    Scalar(String),
    Dictionary(HashMap<String, String>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Index {
    Array(Vec<String>),
    Dictionary(HashMap<String, u32>),
}

type Unit = serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    index: Option<Index>,
    label: Option<HashMap<String, String>>,
    child: Option<HashMap<String, Vec<String>>>,
    coordinates: Option<HashMap<String, Vec<f64>>>,
    unit: Option<HashMap<String, Unit>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dimension {
    category: Category,
    label: Option<String>,
    class: Option<Class>,
}

type Extension = HashMap<String, serde_json::Value>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Role {
    time: Option<Vec<DimensionId>>,
    geo: Option<Vec<DimensionId>>,
    metric: Option<Vec<DimensionId>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Updated {
    Date(chrono::NaiveDate),
    DateTime(chrono::DateTime<chrono::FixedOffset>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Link {
    JsonStat(LinkJsonStat),
    Other(LinkOther),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkJsonStat {
    class: Option<Class>,
    label: Option<String>,
    href: Option<String>,
    extension: Option<Extension>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkOther {
    #[serde(rename="link")]
    linktype: String,
    href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonStat {
    pub version: Version,
    pub class: Class,
    pub label: Option<String>,
    pub id: Option<Vec<String>>,
    pub size: Option<Vec<u32>>,
    pub value: Option<StatValue>,
    pub dimension: Option<HashMap<DimensionId, Dimension>>,
    pub updated: Option<Updated>,
    pub extension: Option<Extension>,
    pub href: Option<String>,
    pub role: Option<Role>,
    pub status: Option<Status>,
    pub category: Option<Category>,
    pub link: Option<HashMap<String, Vec<Link>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonStatDataset {
    pub version: Version,
    pub class: Class,
    pub label: Option<String>,
    pub id: Vec<String>,
    pub size: Vec<u32>,
    pub value: StatValue,
    pub dimension: HashMap<String, Dimension>,
    pub updated: Option<Updated>,
    pub extension: Option<Extension>,
    pub href: Option<String>,
    pub role: Option<Role>,
    pub status: Option<Status>,
}

impl TryFrom<JsonStat> for JsonStatDataset {
    type Error = ();

    fn try_from(x: JsonStat) -> Result<Self, Self::Error> {
        if x.class != Class::Dataset {
            return Err(())
        }

        if x.category.is_some() {
            return Err(())
        }

        Ok(JsonStatDataset {
            version: x.version,
            class: x.class,
            label: x.label,
            id: x.id.unwrap(),
            size: x.size.unwrap(),
            value: x.value.unwrap(),
            dimension: x.dimension.unwrap(),
            updated: x.updated,
            extension: x.extension,
            href: x.href,
            role: x.role,
            status: x.status,
        })
    }
}
