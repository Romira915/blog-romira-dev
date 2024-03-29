use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use std::fmt::Display;

#[async_trait]
pub(crate) trait Fetch: Sized {
    async fn fetch<T>(id: T, is_preview: bool) -> Result<Self>
    where
        T: Display + Send + Sync;
}

pub(crate) trait ArticleTrait {
    fn title(&self) -> String;
    fn href(&self) -> String;
    fn thumbnail_url(&self) -> String;
    fn category(&self) -> Option<String>;
    fn first_published_at(&self) -> Option<DateTime<FixedOffset>>;
}
