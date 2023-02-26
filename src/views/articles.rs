use crate::app::models::article::Article;
use crate::app::{controllers::article, models::article::Articles};
#[cfg(feature = "ssr")]
use crate::settings::CONFIG;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use yew::prelude::*;

#[cfg(feature = "ssr")]
async fn fetch_posts() -> String {
    let client = Client::new();
    let response = client
        .get("https://blog-romira-dev.cdn.newt.so/v1/blog/article")
        .header(
            "Authorization",
            &format!("Bearer {}", CONFIG.newt_api_key()),
        )
        .send()
        .await;

    serde_json::to_string(&response.unwrap().json::<Value>().await.unwrap()).unwrap()
}

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
        async move |_| -> Articles { article::fetch_articles().await.unwrap() },
        ()
    )?
    .unwrap();
    let body = articles.items[0].body.clone().unwrap_or_default();
    let html = Html::from_html_unchecked(body.into());

    Ok(html)
}
