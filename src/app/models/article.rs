use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::settings::{CONFIG, NEWT_BASE_URL, NEWT_CDN_BASE_URL};

use super::{
    author::Author,
    category::Category,
    fields::{Image, Meta, Sys},
    traits::Fetch,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Articles {
    pub(crate) skip: u32,
    pub(crate) limit: u32,
    pub(crate) total: u32,
    pub(crate) items: Vec<Article>,
}

impl Articles {
    pub(crate) async fn fetch(is_preview: bool) -> Result<Self> {
        let (base_url, api_token) = if is_preview {
            (NEWT_BASE_URL, CONFIG.newt_api_token())
        } else {
            (NEWT_CDN_BASE_URL, CONFIG.newt_cdn_api_token())
        };

        let client = Client::new();
        log::info!("fetch articles.");
        let response = client
            .get(format!("{base_url}/blog/article"))
            .header("Authorization", &format!("Bearer {}", api_token))
            .send()
            .await?;

        let body = response.text().await?;

        serde_json::from_str(&body).with_context(|| format!("Failed to json parse. json: {}", body))
    }
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

impl Article {
    pub(crate) async fn fetch<T>(article_id: T, is_preview: bool) -> Result<Self>
    where
        T: std::fmt::Display + Send + Sync,
    {
        let (base_url, api_token) = if is_preview {
            (NEWT_BASE_URL, CONFIG.newt_api_token())
        } else {
            (NEWT_CDN_BASE_URL, CONFIG.newt_cdn_api_token())
        };

        let client = Client::new();
        log::info!("fetch article. article_id: {}", article_id);
        let response = client
            .get(format!("{base_url}/blog/article/{article_id}"))
            .header("Authorization", &format!("Bearer {}", api_token))
            .send()
            .await?;

        let body = response.text().await?;

        serde_json::from_str(&body).with_context(|| format!("Failed to json parse. json: {}", body))
    }
}
