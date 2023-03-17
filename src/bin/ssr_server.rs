use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use axum::body::{Body, StreamBody};
use axum::error_handling::HandleError;
use axum::extract::Query;
use axum::handler::Handler;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use blog_romira_dev::prelude::{ServerApp, ServerAppProps};
use blog_romira_dev::routes::Route;
use blog_romira_dev::settings::CONFIG;
use clap::Parser;
use futures::stream::{self, StreamExt};
use hyper::server::Server;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use yew::platform::Runtime;

use blog_romira_dev::app::controllers::article_controller::get_article_ogp_tag;

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
    Extension((index_html_before, index_html_after)): Extension<(String, String)>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let url = url.uri().to_string();

    log::debug!("index {:#?}", index_html_before.split_once("<head>"));
    let (index_html_top, index_html_head) = index_html_before.split_once("<head>").unwrap();
    let mut index_html_top = index_html_top.to_owned();
    index_html_top.push_str(r###"<head prefix=og: http://ogp.me/ns#>"###);

    let route = Route::from_str(&url);

    match route {
        Ok(Route::Article { id }) => {
            log::debug!("OGP Setting {}", id);
            let meta = get_article_ogp_tag(&id, &url, false)
                .await
                .unwrap_or_default();
            index_html_top.push_str(&meta);
        }
        _ => (),
    }

    let index_html_before = format!("{}{}", index_html_top, index_html_head);

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    StreamBody::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
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

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| CONFIG.rust_log().into()),
        )
        .try_init()?;

    let exec = Executor::default();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {e}"),
        )
    };

    let app = Router::new()
        .route("/api/test", get(|| async move { "Hello World" }))
        .fallback(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    render
                        .layer(Extension((
                            index_html_before.clone(),
                            index_html_after.clone(),
                        )))
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            handle_error,
        ));

    let address = "0.0.0.0:8080".parse()?;
    tracing::info!(message = "listening", addr = ?address);
    log::info!("Browser to {}", CONFIG.app_origin);

    Server::bind(&address)
        .executor(exec)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
