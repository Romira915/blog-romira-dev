use crate::app::models::cms_article::CMSArticle;
use crate::app::{controllers::article_controller, models::cms_article::CMSArticles};
#[cfg(feature = "ssr")]
use crate::settings::CONFIG;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

#[cfg(feature = "ssr")]
async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

#[function_component]
pub fn Posts() -> HtmlResult {
    // let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())?.unwrap();
    let articles = use_prepared_state!(
        async move |_| -> CMSArticles { article_controller::fetch_articles().await.unwrap() },
        ()
    )?
    .unwrap();
    let body = articles.items[0].body.clone().unwrap_or_default();
    let html = Html::from_html_unchecked(body.into());

    Ok(html)
}
