use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::{cms_article::CMSArticle, traits::ArticleTrait, wp_article::WpArticle};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum ArticleType {
    CMS,
    WordPress,
}

impl Default for ArticleType {
    fn default() -> Self {
        Self::CMS
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct Articles {
    pub(crate) items: Vec<Article>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct Article {
    pub(crate) title: String,
    pub(crate) href: String,
    pub(crate) thumbnail_url: String,
    pub(crate) categorie: Option<String>,
    pub(crate) first_published_at: Option<DateTime<FixedOffset>>,
    pub(crate) r#type: ArticleType,
}

impl From<&CMSArticle> for Article {
    fn from(cms_article: &CMSArticle) -> Self {
        Self {
            title: cms_article.title(),
            href: cms_article.href(),
            thumbnail_url: cms_article.thumbnail_url(),
            categorie: cms_article.category(),
            first_published_at: cms_article.first_published_at(),
            r#type: ArticleType::CMS,
        }
    }
}

impl From<&WpArticle> for Article {
    fn from(wp_article: &WpArticle) -> Self {
        Self {
            title: wp_article.title(),
            href: wp_article.href(),
            thumbnail_url: wp_article.thumbnail_url(),
            categorie: wp_article.category(),
            first_published_at: wp_article.first_published_at(),
            r#type: ArticleType::WordPress,
        }
    }
}
