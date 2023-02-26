use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{author::Author, category::Category};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Articles {
    pub(crate) skip: u32,
    pub(crate) limit: u32,
    pub(crate) total: u32,
    pub(crate) items: Vec<Article>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Article {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(rename = "_sys")]
    pub(crate) sys: Value,
    pub(crate) title: String,
    pub(crate) slug: String,
    pub(crate) meta: Option<Meta>,
    pub(crate) body: Option<String>,
    pub(crate) cover_image: Option<Image>,
    pub(crate) author: Option<Author>,
    pub(crate) categries: Option<Vec<Category>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Meta {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) og_image: Option<Image>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Image {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    pub(crate) src: String,
    pub(crate) file_type: String,
    pub(crate) file_size: u32,
    pub(crate) file_name: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}
