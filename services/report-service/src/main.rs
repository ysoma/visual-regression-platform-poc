use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

#[derive(Serialize, sqlx::FromRow)]
struct TestRun {
    id: Uuid,
    status: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct Screenshot {
    id: Uuid,
    test_run_id: Uuid,
    url: String,
    s3_key: Option<String>,
    status: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/reports", get(list_reports))
        .route("/reports/:id", get(get_report))
        .route("/health", get(health_check))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Report Service listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn list_reports(State(state): State<AppState>) -> Json<Vec<TestRun>> {
    let runs =
        sqlx::query_as::<_, TestRun>("SELECT id, status FROM test_runs ORDER BY created_at DESC")
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();

    Json(runs)
}

async fn get_report(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<Vec<Screenshot>> {
    let screenshots = sqlx::query_as::<_, Screenshot>(
        "SELECT id, test_run_id, url, s3_key, status FROM screenshots WHERE test_run_id = $1",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(screenshots)
}
