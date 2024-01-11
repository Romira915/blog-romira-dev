use std::fmt::Display;

use chrono::FixedOffset;
use reqwest::Client;

use crate::app::models::article::{Article, Articles};
use crate::app::models::cms_article::{CMSArticle, CMSArticles};
use crate::app::models::traits::ArticleTrait;
use crate::app::models::wp_article::WpArticles;
use crate::const_value::{HOUR, JST_TZ};
use crate::settings::CONFIG;
use anyhow::{Context, Result};

// #[cfg(feature = "ssr")]
pub(crate) async fn fetch_articles() -> Result<Articles> {
    let cms_articles = CMSArticles::fetch(false).await?;
    let wp_articles = match WpArticles::fetch().await {
        Ok(wp_articles) => wp_articles,
        Err(e) => {
            log::warn!("Failed to fetch wp_articles. error: {}", e);
            WpArticles::default()
        }
    };

    let mut articles: Vec<Article> = cms_articles
        .items
        .iter()
        .map(|a| Article::from(a))
        .collect();
    articles.append(
        &mut wp_articles
            .articles
            .iter()
            .map(|a| Article::from(a))
            .collect(),
    );
    articles.sort_by_key(|a| a.first_published_at);
    articles.reverse();

    Ok(Articles { items: articles })
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_public_article(article_id: &str) -> Result<CMSArticle> {
    CMSArticle::fetch(article_id, false).await
}

#[cfg(feature = "ssr")]
pub(crate) async fn fetch_preview_article(article_id: &str) -> Result<CMSArticle> {
    CMSArticle::fetch(article_id, true).await
}

#[cfg(feature = "ssr")]
fn title_tag<T>(title: T) -> String
where
    T: Display,
{
    format!("<title>{}</title>", title)
}

#[cfg(feature = "ssr")]
pub fn home_meta_tag<T>(url: T) -> Result<String>
where
    T: Display,
{
    let mut meta = String::new();
    meta.push_str(&title_tag("Romira's develop blog"));
    meta.push_str(r###"<meta name="description" content="Rustaceanによる開発ブログです．技術共有や個人開発の進捗などを発信します．">"###);
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
        r###"<meta property="og:description" content="Rustaceanによる開発ブログです．技術共有や個人開発の進捗などを発信します．" />
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
pub async fn article_meta_tag<T>(article_id: &str, url: T, is_preview: bool) -> Result<String>
where
    T: Display,
{
    let article = if is_preview {
        fetch_preview_article(article_id).await?
    } else {
        fetch_public_article(article_id).await?
    };

    let mut meta = String::new();
    meta.push_str(&title_tag(&article.title));
    meta.push_str(&format!(
        r###"<meta name="description" content="{}">
        "###,
        article
            .meta
            .as_ref()
            .map(|m| m.description.as_str())
            .unwrap_or_default()
    ));
    meta.push_str(&format!(
        r###"<meta name="date" content="{}">
                    "###,
        article
            .sys
            .updated_at
            .with_timezone(&FixedOffset::east_opt(JST_TZ * HOUR).unwrap())
            .to_rfc3339(),
    ));
    meta.push_str(&format!(
        r###"<meta name="creation_date" content="{}">
                    "###,
        article
            .sys
            .raw
            .first_published_at
            .map(|d| d
                .with_timezone(&FixedOffset::east_opt(JST_TZ * HOUR).unwrap())
                .to_rfc3339())
            .unwrap_or_default(),
    ));
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
