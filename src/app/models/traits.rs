use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Display;

#[async_trait]
pub(crate) trait Fetch: Sized {
    async fn fetch<T>(id: T, is_preview: bool) -> Result<Self>
    where
        T: Display + Send + Sync;
}
