use std::fmt::Display;

use reqwest::Client;

use crate::app::models::article::{Article, Articles};
use crate::settings::{CONFIG, NEWT_BASE_URL, NEWT_CDN_BASE_URL};
use anyhow::{Context, Result};

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_articles() -> Result<Articles> {
    Articles::fetch(false).await
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_article_with_public(article_id: &str) -> Result<Article> {
    Article::fetch(article_id, false).await
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_article_with_preview(article_id: &str) -> Result<Article> {
    Article::fetch(article_id, true).await
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
        r###"<meta property="og:url" content="{}{}" />
        "###,
        CONFIG.app_origin, url
    ));
    meta.push_str(&format!(
        r###"<meta property="og:type" content="article" />
        "###
    ));
    meta.push_str(&format!(
        r###"<meta property="og:title" content="{}" />
        "###,
        article.title
    ));
    meta.push_str(&format!(
        r###"<meta property="og:description" content="{}" />
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m.description.as_str())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta property="og:site_name" content="romira's develop blog" />
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:image" content="{}" />
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
        r###"<meta name="twitter:card" content="summary_large_image" />
        "###
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:title" content="{}" />
        "###,
        article.title
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:description" content="{}" />
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m.description.as_str())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:image" content="{}" />
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
        r###"<meta name="twitter:creator" content="@Romira915" />
        "###,
    ));

    Ok(meta)
}
