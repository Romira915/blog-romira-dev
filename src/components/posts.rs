use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct PostProps {}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    // let request_handle = use_async_with_options(
    //     async move {
    //         let client = Client::new();
    //         let response = client
    //             .get("https://blog-romira-dev.cdn.newt.so/v1/blog/article")
    //             .header(
    //                 "Authorization",
    //                 "Bearer B3UcPsaT0eaXTlfWT2MOBwW5MLgVuBB9c77p-HqrvvE",
    //             )
    //             .send()
    //             .await;
    //         response
    //             .unwrap()
    //             .json::<Value>()
    //             .await
    //             .map_err(|e| e.to_string())
    //     },
    //     UseAsyncOptions::enable_auto(),
    // );

    html! {
        <div>
        {
            // request_handle.data.clone().unwrap_or_default().to_string()
            "test"
        }
        </div>
    }
}
