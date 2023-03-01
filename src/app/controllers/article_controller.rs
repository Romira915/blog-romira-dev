use reqwest::Client;

use crate::app::models::article::{Article, Articles};
use crate::settings::{CONFIG, NEWT_CDN_BASE_URL};
use anyhow::{Context, Result};

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_articles() -> Result<Articles, serde_json::Error> {
    use crate::test_data::{self, ARTICLE_JSON};

    let client = Client::new();
    log::info!("fetch articles.");
    // let response = client
    //     .get(format!("{NEWT_CDN_BASE_URL}/blog/article"))
    //     .header(
    //         "Authorization",
    //         &format!("Bearer {}", CONFIG.newt_api_key()),
    //     )
    //     .send()
    //     .await;

    serde_json::from_str::<Articles>(ARTICLE_JSON.trim())

    // Ok(response.unwrap().json::<Articles>().await.unwrap())
}

// #[cfg(feature = "ssr")]
pub(crate) async fn fetch_article(article_id: &str) -> Result<Article> {
    let client = Client::new();
    log::info!("fetch article. article_id: {}", article_id);
    let response = client
        .get(format!("{NEWT_CDN_BASE_URL}/blog/article/{article_id}"))
        .header(
            "Authorization",
            &format!("Bearer {}", CONFIG.newt_api_key()),
        )
        .send()
        .await;

    response
        .with_context(|| format!("Failed to http request. article_id: {}", article_id))?
        .json()
        .await
        .context("Failed to json parse")
}
