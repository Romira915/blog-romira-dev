use serde::{Deserialize, Serialize};


use super::{
    author::Author,
    category::Category,
    fields::{Image, Meta, Sys},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Articles {
    pub(crate) skip: u32,
    pub(crate) limit: u32,
    pub(crate) total: u32,
    pub(crate) items: Vec<Article>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Article {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(rename = "_sys")]
    pub(crate) sys: Sys,
    pub(crate) title: String,
    pub(crate) slug: String,
    pub(crate) meta: Option<Meta>,
    pub(crate) body: Option<String>,
    pub(crate) cover_image: Option<Image>,
    pub(crate) author: Option<Author>,
    pub(crate) categries: Option<Vec<Category>>,
}
