use anyhow::Result;

use crate::app::models::author::Author;

pub(crate) async fn fetch_author(author_id: &str) -> Result<Author> {
    Author::fetch(author_id).await
}
