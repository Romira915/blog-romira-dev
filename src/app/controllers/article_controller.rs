use reqwest::Client;

use crate::app::models::article::{Article};
use crate::settings::{CONFIG, NEWT_BASE_URL, NEWT_CDN_BASE_URL};
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
pub(crate) async fn fetch_article(article_id: &str, is_preview: bool) -> Result<Article> {
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
        .await;

    response
        .with_context(|| format!("Failed to http request. article_id: {}", article_id))?
        .json()
        .await
        .context("Failed to json parse")
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_article_with_public(article_id: &str) -> Result<Article> {
    fetch_article(article_id, false).await
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_article_with_preview(article_id: &str) -> Result<Article> {
    fetch_article(article_id, true).await
}
