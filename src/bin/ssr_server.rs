use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use axum::body::Body;
use axum::error_handling::HandleError;
use axum::extract::{Query, State};
use axum::handler::Handler;
use axum::http::{Request, StatusCode, Uri};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};
use blog_romira_dev::prelude::{ServerApp, ServerAppProps};
use blog_romira_dev::routes::Route;
use blog_romira_dev::settings::{init_logger, CONFIG};
use clap::Parser;
use futures::stream::{self, StreamExt};
use hyper::header::CACHE_CONTROL;
use hyper::HeaderMap;
use tower::{MakeService, ServiceExt};
use tower_http::services::ServeDir;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use yew::platform::Runtime;

use blog_romira_dev::app::controllers::article_controller;

type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

// We use jemalloc as it produces better performance.
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[arg(short, long)]
    dir: PathBuf,
    #[arg(short, long)]
    log_file: Option<PathBuf>,
}

async fn render(
    uri: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> impl IntoResponse {
    let path = uri.path().to_string();

    log::debug!("index {:#?}", index_html_before.split_once("</head>"));
    // <head>タグがビルド時に消えるため暫定対応
    let (index_html_top, index_html_head) = index_html_before.split_once("</head>").unwrap();
    let mut index_html_top = index_html_top.to_owned();
    // <head>タグがビルド時に消えるため暫定対応
    // index_html_top.push_str(r###"<head prefix=og: http://ogp.me/ns#>"###);

    let route = Route::from_str(&path);

    let meta = match &route {
        Ok(Route::Article { id }) => {
            log::debug!("Article OGP Setting {}", id);
            article_controller::article_meta_tag(&id, &path, false).await
        }
        Ok(Route::Preview { id }) => {
            log::debug!("Preview OGP Setting {}", id);
            article_controller::article_meta_tag(&id, &path, true).await
        }
        Ok(Route::Home) => {
            log::debug!("Home OGP Setting");
            article_controller::home_meta_tag(&path)
        }
        _ => Ok("".to_string()),
    };
    match meta {
        Ok(meta) => index_html_top.push_str(&meta),
        Err(e) => log::warn!("{:#}", e),
    }
    // <head>タグがビルド時に消えるため暫定対応
    index_html_top.push_str(r###"</head>"###);

    let headers = match &route {
        Ok(Route::Article { id }) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                CACHE_CONTROL,
                format!(
                    "max-age={}, s-maxage={}",
                    60 * 60 * 24 * 365,
                    60 * 60 * 24 * 365
                )
                .parse()
                .unwrap(),
            );
            headers
        }
        Ok(Route::Preview { id }) => {
            let mut headers = HeaderMap::new();
            headers.insert(CACHE_CONTROL, "no-cache, no-store".parse().unwrap());
            headers
        }
        Ok(Route::Home) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                CACHE_CONTROL,
                format!(
                    "max-age={}, s-maxage={}",
                    60 * 60 * 24 * 365,
                    60 * 60 * 24 * 365
                )
                .parse()
                .unwrap(),
            );
            headers
        }
        _ => HeaderMap::new(),
    };

    let index_html_before = format!("{}{}", index_html_top, index_html_head);

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: path.into(),
        queries,
    });

    let mut body = index_html_before;
    body.push_str(&renderer.render().await);
    body.push_str(&index_html_after);

    // StreamBody::new(
    //     stream::once(async move { index_html_before })
    //         .chain(renderer.render_stream())
    //         .chain(stream::once(async move { index_html_after }))
    //         .map(Result::<_, Infallible>::Ok),
    // )
    (headers, Html(body))
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let opts = Opt::parse();

    // tracing_subscriber::fmt()
    //     .with_env_filter(
    //         EnvFilter::try_from_default_env().unwrap_or_else(|_| CONFIG.rust_log().into()),
    //     )
    //     .try_init()?;
    init_logger(CONFIG.rust_log_to_log_level_filter(), None)?;

    let exec = Executor::default();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    // let handle_error = |e| async move {
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         format!("error occurred: {e}"),
    //     )
    // };
    //
    // let app = Router::new().fallback_service(HandleError::new(
    //     ServeDir::new(opts.dir)
    //         .append_index_html_on_directories(false)
    //         .fallback(
    //             get(render)
    //                 .with_state((index_html_before.clone(), index_html_after.clone()))
    //                 .into_service()
    //                 .map_err(|err| -> std::io::Error { match err {} }),
    //         ),
    //     handle_error,
    // ));

    let app = Router::new().fallback_service(
        ServeDir::new(opts.dir)
            .append_index_html_on_directories(false)
            .fallback(
                get(render).with_state((index_html_before.clone(), index_html_after.clone())),
            ),
    );

    let address = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(message = "listening", addr = ?address);
    log::info!("Browser to {}", CONFIG.app_origin);

    axum::serve(listener, app).await?;

    Ok(())
}
