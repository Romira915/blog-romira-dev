use std::fmt::Display;

use reqwest::Client;

use crate::app::models::article::{Article, Articles};
use crate::settings::{CONFIG, NEWT_BASE_URL, NEWT_CDN_BASE_URL};
use anyhow::{Context, Result};

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_articles() -> Result<Articles> {
    use crate::test_data::{self, ARTICLE_JSON};

    let client = Client::new();
    log::info!("fetch articles.");
    let response = client
        .get(format!("{NEWT_CDN_BASE_URL}/blog/article"))
        .header(
            "Authorization",
            &format!("Bearer {}", CONFIG.newt_cdn_api_token()),
        )
        .send()
        .await;

    Ok(response?.json::<Articles>().await.unwrap_or_default())
}

#[cfg(feature = "ssr")]
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

#[cfg(feature = "ssr")]
pub async fn get_article_ogp_tag<T>(article_id: &str, url: T, is_preview: bool) -> Result<String>
where
    T: Display,
{
    let article = if is_preview {
        fetch_article_with_preview(article_id).await?
    } else {
        fetch_article_with_public(article_id).await?
    };

    let mut meta = String::new();
    meta.push_str(&format!(
        r###"<meta property="og:url" content="{}{}">
        "###,
        CONFIG.app_origin, url
    ));
    meta.push_str(&format!(
        r###"<meta property="og:type" content="article">
        "###
    ));
    meta.push_str(&format!(
        r###"<meta property="og:title" content="{}">
        "###,
        article.title
    ));
    meta.push_str(&format!(
        r###"<meta property="og:description" content="{}">
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m.description.as_str())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta property="og:site_name" content="romira's develop blog">
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:image" content="{}">
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m
                .og_image
                .as_ref()
                .map(|i| i.src.as_str())
                .unwrap_or_default())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:card" content="summary_large_image">
        "###
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:title" content="{}">
        "###,
        article.title
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:description" content="{}">
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m.description.as_str())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:image:src" content="{}">
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m
                .og_image
                .as_ref()
                .map(|i| i.src.as_str())
                .unwrap_or_default())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:creator" content="@Romira915">
        "###,
    ));

    Ok(meta)
}
