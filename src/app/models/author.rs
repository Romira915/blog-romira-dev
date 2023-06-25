use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{app::models::author, const_value::NEWT_BASE_URL, settings::CONFIG};

use super::fields::{Image, Sys};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Author {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(rename = "_sys")]
    pub(crate) sys: Sys,
    pub(crate) full_name: String,
    #[serde(rename = "profileImage")]
    pub(crate) profile_image_id: Option<Image>,
    pub(crate) biography: Option<String>,
}

impl Author {
    pub(crate) async fn fetch<T>(author_id: T) -> Result<Self>
    where
        T: std::fmt::Display + Send + Sync,
    {
        let (base_url, api_token) = (NEWT_BASE_URL, CONFIG.newt_api_token());
        let client = Client::new();
        log::info!("fetch author. author_id: {}", author_id);
        let response = client
            .get(format!("{base_url}/blog/author/{author_id}"))
            .header("Authorization", &format!("Bearer {}", api_token))
            .send()
            .await?;

        let body = response.text().await?;

        serde_json::from_str(&body).with_context(|| format!("Failed to json parse. json: {}", body))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AuthorInArticle {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(rename = "_sys")]
    pub(crate) sys: Sys,
    pub(crate) full_name: String,
    #[serde(rename = "profileImage")]
    pub(crate) profile_image_id: Option<String>,
    pub(crate) biography: Option<String>,
}
