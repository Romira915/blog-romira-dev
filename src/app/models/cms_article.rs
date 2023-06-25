use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    const_value::{HOUR, JST_TZ, NEWT_BASE_URL, NEWT_CDN_BASE_URL, THUMBNAIL_NO_IMAGE_URL},
    settings::CONFIG,
};

use super::{
    author::{Author, AuthorInArticle},
    category::Category,
    fields::{Image, Meta, Sys},
    traits::{ArticleTrait, Fetch},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CMSArticles {
    pub(crate) skip: u32,
    pub(crate) limit: u32,
    pub(crate) total: u32,
    pub(crate) items: Vec<CMSArticle>,
}

impl CMSArticles {
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
pub(crate) struct CMSArticle {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(rename = "_sys")]
    pub(crate) sys: Sys,
    pub(crate) title: String,
    pub(crate) slug: String,
    pub(crate) meta: Option<Meta>,
    pub(crate) body: Option<String>,
    pub(crate) cover_image: Option<Image>,
    pub(crate) author: Option<AuthorInArticle>,
    pub(crate) categries: Option<Vec<Category>>,
}

impl CMSArticle {
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

impl ArticleTrait for CMSArticle {
    fn title(&self) -> String {
        self.title.to_string()
    }

    fn href(&self) -> String {
        format!("articles/{}", self.id)
    }

    fn thumbnail_url(&self) -> String {
        self.cover_image
            .as_ref()
            .map_or(THUMBNAIL_NO_IMAGE_URL.to_string(), |img| {
                img.src.to_string()
            })
    }

    fn categorie(&self) -> Option<String> {
        let category = self.categries.as_ref().map(|c| c.first());

        match category {
            Some(Some(category)) => Some(category.name.to_string()),
            _ => None,
        }
    }

    fn first_published_at(&self) -> Option<String> {
        self.sys.raw.first_published_at.as_ref().map(|date| {
            date.with_timezone(&FixedOffset::east_opt(JST_TZ * HOUR).unwrap())
                .date_naive()
                .format("%Y年%m月%d日")
                .to_string()
        })
    }
}
