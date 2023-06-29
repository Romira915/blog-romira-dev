use crate::const_value::PRTIMES_WP_BASE_URL;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct Category {
    pub(crate) id: u64,
    pub(crate) count: u64,
    pub(crate) description: String,
    pub(crate) link: String,
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) taxonomy: String,
    pub(crate) parent: u64,
    pub(crate) meta: Vec<String>,
    pub(crate) _links: Links,
}

impl Category {
    pub(crate) async fn fetch(id: u64) -> Result<Self> {
        let client = Client::new();
        log::info!("fetch wp_category. id: {}", id);
        let response = client
            .get(format!(
                "{PRTIMES_WP_BASE_URL}/wp-json/wp/v2/categories/{id}"
            ))
            .send()
            .await?;

        let body = response.text().await?;

        serde_json::from_str(&body).with_context(|| format!("Failed to json parse. json: {}", body))
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct Links {
    #[serde(rename = "self")]
    pub(crate) self_: Vec<Link>,
    pub(crate) collection: Vec<Link>,
    pub(crate) about: Vec<Link>,
    #[serde(rename = "wp:post_type")]
    pub(crate) wp_post_type: Vec<Link>,
    pub(crate) curies: Vec<Cury>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct Link {
    pub(crate) href: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct Cury {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) templated: bool,
}
