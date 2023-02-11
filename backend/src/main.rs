use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, response::Html, routing::get, Router};
use blog_romira_dev_backend::CONFIG;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(CONFIG.database_url_as_ref())
        .await
        .expect("Can't connect to database");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route(
            "/psql",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .with_state(pool);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    let r = sqlx::query!("SELECT id, name FROM test1")
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(format!("{:?} {:?}", r.id, r.name))
}

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    let r = sqlx::query!("SELECT id, name FROM test1")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    Ok(format!("{:?} {:?}", r.id, r.name))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
