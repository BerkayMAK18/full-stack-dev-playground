//crates
use std::{sync::Arc, env};
use axum::{Router, routing::{get, post}, Extension, serve};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use tracing_subscriber;

//models
use apiserver::{db, handlers};
use apiserver::models::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;
    let pool = db::init_pool(&database_url).await?;

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))

        .route("/tasks", get(handlers::tasks::list_tasks).post(handlers::tasks::create_task))
        .route("/tasks/{id}", get(handlers::tasks::get_task).put(handlers::tasks::update_task).delete(handlers::tasks::delete_task))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(Extension(state));

    let addr = "127.0.0.1:8080";
    tracing::info!("Listening on https://{}", addr);
    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app).await?;



    Ok(())
}
