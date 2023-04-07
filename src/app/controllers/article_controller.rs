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
pub(crate) async fn fetch_public_article(article_id: &str) -> Result<Article> {
    Article::fetch(article_id, false).await
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_preview_article(article_id: &str) -> Result<Article> {
    Article::fetch(article_id, true).await
}

#[cfg(feature = "ssr")]
pub fn home_ogp_tag<T>(url: T) -> Result<String>
where
    T: Display,
{
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
        r###"<meta property="og:title" content="Romira's develop blog" />
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:description" content="Rustaceanによる開発ブログです．技術共有や個人開発などを発信します．" />
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:site_name" content="Romira's develop blog" />
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:image" content="https://blog-romira.imgix.net/46cea3d7-14ce-45bf-9d1e-52d1df39f2d2/romira'sdevelopblog.png" />
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:creator" content="@Romira915" />
        "###,
    ));

    Ok(meta)
}

#[cfg(feature = "ssr")]
pub async fn article_ogp_tag<T>(article_id: &str, url: T, is_preview: bool) -> Result<String>
where
    T: Display,
{
    let article = if is_preview {
        fetch_preview_article(article_id).await?
    } else {
        fetch_public_article(article_id).await?
    };

    let mut meta = String::new();
    meta.push_str(&format!(
        r###"<meta property="og:url" content="{}{}" />
        "###,
        CONFIG.app_origin, url
    ));
    meta.push_str(&format!(
        r###"<meta property="og:type" content="blog" />
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
        r###"<meta property="og:site_name" content="Romira's develop blog" />
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
