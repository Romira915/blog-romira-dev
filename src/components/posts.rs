use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;

// #[cfg(feature = "ssr")]
// async fn fetch_posts() -> Value {
//     let client = Client::new();
//     let response = client
//         .get("https://blog-romira-dev.cdn.newt.so/v1/blog/article")
//         .header(
//             "Authorization",
//             "Bearer B3UcPsaT0eaXTlfWT2MOBwW5MLgVuBB9c77p-HqrvvE",
//         )
//         .send()
//         .await;

//     response.unwrap().json().await.unwrap()
// }

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
    let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())?.unwrap();

    Ok(html! {
        <div>{"Random UUID: "}{uuid}</div>
    })
}
