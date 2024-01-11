use crate::{
    app::models::wp_category::Category,
    const_value::{HOUR, JST_TZ, PRTIMES_WP_AUTHOR_ID, PRTIMES_WP_BASE_URL},
};
use anyhow::{Context, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use futures::{future, stream::StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::traits::ArticleTrait;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WpArticles {
    pub(crate) articles: Vec<WpArticle>,
}

impl WpArticles {
    pub(crate) async fn fetch() -> Result<WpArticles> {
        let client = Client::new();
        log::info!("fetch wp_articles.");
        let response = client
            .get(format!(
                "{PRTIMES_WP_BASE_URL}/wp-json/wp/v2/posts?author={PRTIMES_WP_AUTHOR_ID}"
            ))
            .send()
            .await?;

        let body = response.text().await?;

        let mut wp_articles = serde_json::from_str::<Vec<WpArticle>>(&body)
            .with_context(|| format!("Failed to json parse. json: {}", body))?;
        for wp_article in &mut wp_articles {
            let categorys = wp_article
                .categories
                .iter()
                .map(|category_id| async { Category::fetch(*category_id).await })
                .collect::<futures::stream::FuturesUnordered<_>>()
                .filter(|c| future::ready(c.is_ok()))
                .map(|c| c.unwrap())
                .collect()
                .await;
            wp_article.category_names = categorys;
        }

        Ok(Self {
            articles: wp_articles,
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct WpArticle {
    #[serde(rename = "id")]
    pub(crate) id: u64,
    pub(crate) date: NaiveDateTime,
    pub(crate) date_gmt: NaiveDateTime,
    pub(crate) guid: Guid,
    pub(crate) modified: String,
    pub(crate) modified_gmt: String,
    pub(crate) slug: String,
    pub(crate) status: String,
    pub(crate) r#type: String,
    pub(crate) link: String,
    pub(crate) title: Title,
    pub(crate) content: Content,
    pub(crate) excerpt: Excerpt,
    pub(crate) author: u64,
    pub(crate) featured_media: u64,
    pub(crate) comment_status: String,
    pub(crate) ping_status: String,
    pub(crate) sticky: bool,
    pub(crate) template: Option<String>,
    pub(crate) format: String,
    pub(crate) meta: Meta,
    pub(crate) categories: Vec<u64>,
    pub(crate) tags: Vec<u64>,
    pub(crate) jetpack_publicize_connections: Vec<String>,
    pub(crate) jetpack_featured_media_url: String,
    pub(crate) jetpack_likes_enabled: bool,
    pub(crate) jetpack_sharing_enabled: bool,
    pub(crate) jetpack_shortlink: String,
    #[serde(rename = "_links")]
    pub(crate) links: Links,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) category_names: Vec<Category>,
}

impl ArticleTrait for WpArticle {
    fn title(&self) -> String {
        self.title.rendered.to_string()
    }

    fn href(&self) -> String {
        self.link.to_string()
    }

    fn thumbnail_url(&self) -> String {
        self.jetpack_featured_media_url.to_string()
    }

    fn category(&self) -> Option<String> {
        Some(
            self.category_names
                .first()
                .map_or("".to_string(), |c| c.name.to_string()),
        )
    }

    fn first_published_at(&self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        Some(DateTime::<FixedOffset>::from_utc(
            self.date_gmt,
            FixedOffset::east_opt(JST_TZ * HOUR).unwrap(),
        ))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Guid {
    pub(crate) rendered: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Title {
    pub(crate) rendered: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Content {
    pub(crate) rendered: String,
    pub(crate) protected: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Excerpt {
    pub(crate) rendered: String,
    pub(crate) protected: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Meta {
    #[serde(rename = "_coblocks_attr")]
    pub(crate) coblocks_attr: String,
    #[serde(rename = "_coblocks_dimensions")]
    pub(crate) coblocks_dimensions: String,
    #[serde(rename = "_coblocks_responsive_height")]
    pub(crate) coblocks_responsive_height: String,
    #[serde(rename = "_coblocks_accordion_ie_support")]
    pub(crate) coblocks_accordion_ie_support: String,
    pub(crate) advanced_seo_description: String,
    pub(crate) jetpack_seo_html_title: String,
    pub(crate) jetpack_seo_noindex: bool,
    pub(crate) swell_btn_cv_data: String,
    pub(crate) jetpack_publicize_message: String,
    pub(crate) jetpack_publicize_feature_enabled: bool,
    pub(crate) jetpack_social_post_already_shared: bool,
    pub(crate) jetpack_social_options: JetpackSocialOptions,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct JetpackSocialOptions {
    pub(crate) image_generator_settings: ImageGeneratorSettings,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct ImageGeneratorSettings {
    pub(crate) template: String,
    pub(crate) enabled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Links {
    #[serde(rename = "self")]
    pub(crate) self_: Vec<Link>,
    pub(crate) collection: Vec<Link>,
    pub(crate) about: Vec<Link>,
    pub(crate) author: Vec<Link>,
    pub(crate) replies: Vec<Link>,
    #[serde(rename = "version-history")]
    pub(crate) version_history: Vec<Link>,
    #[serde(rename = "predecessor-version")]
    pub(crate) predecessor_version: Vec<Link>,
    #[serde(rename = "wp:featuredmedia")]
    pub(crate) wp_featuredmedia: Vec<Link>,
    #[serde(rename = "wp:attachment")]
    pub(crate) wp_attachment: Vec<Link>,
    #[serde(rename = "wp:term")]
    pub(crate) wp_term: Vec<TermLink>,
    pub(crate) curies: Vec<CuriLink>,
}

/// NOTE: 型定義がめんどくさいのでスキーマを省略している．
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct Link {
    pub(crate) href: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct TermLink {
    pub(crate) taxonomy: String,
    pub(crate) embeddable: bool,
    pub(crate) href: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct CuriLink {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) templated: bool,
}
