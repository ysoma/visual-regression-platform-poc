use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    worker_url: String,
}

#[derive(Deserialize)]
struct CreateTestRequest {
    urls: Vec<String>,
}

#[derive(Serialize)]
struct TestRunResponse {
    id: Uuid,
    status: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let worker_url =
        std::env::var("WORKER_URL").unwrap_or_else(|_| "http://worker:3000".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Create table if not exists (simplistic migration for now)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS test_runs (
            id UUID PRIMARY KEY,
            status TEXT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create test_runs table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS screenshots (
            id UUID PRIMARY KEY,
            test_run_id UUID REFERENCES test_runs(id),
            url TEXT NOT NULL,
            s3_key TEXT,
            status TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create screenshots table");

    let state = Arc::new(AppState {
        db: pool,
        worker_url,
    });

    let app = Router::new()
        .route("/tests", post(create_test_run))
        .route("/health", get(health_check))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Orchestrator listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn create_test_run(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTestRequest>,
) -> Json<TestRunResponse> {
    let test_run_id = Uuid::new_v4();

    // Insert test run
    sqlx::query("INSERT INTO test_runs (id, status) VALUES ($1, $2)")
        .bind(test_run_id)
        .bind("running")
        .execute(&state.db)
        .await
        .unwrap();

    // Spawn task to process urls
    let state_clone = state.clone();
    let urls = payload.urls.clone();
    tokio::spawn(async move {
        process_test_run(state_clone, test_run_id, urls).await;
    });

    Json(TestRunResponse {
        id: test_run_id,
        status: "running".to_string(),
    })
}

async fn process_test_run(state: Arc<AppState>, test_run_id: Uuid, urls: Vec<String>) {
    let client = reqwest::Client::new();

    for url in urls {
        let screenshot_id = Uuid::new_v4();

        // Record attempt
        let _ = sqlx::query(
            "INSERT INTO screenshots (id, test_run_id, url, status) VALUES ($1, $2, $3, $4)",
        )
        .bind(screenshot_id)
        .bind(test_run_id)
        .bind(&url)
        .bind("pending")
        .execute(&state.db)
        .await;

        // Call worker
        let res = client
            .post(format!("{}/screenshot", state.worker_url))
            .json(&serde_json::json!({ "url": url }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.unwrap_or_default();
                    let s3_key = body["s3Key"].as_str().unwrap_or("");

                    let _ = sqlx::query(
                        "UPDATE screenshots SET status = $1, s3_key = $2 WHERE id = $3",
                    )
                    .bind("completed")
                    .bind(s3_key)
                    .bind(screenshot_id)
                    .execute(&state.db)
                    .await;
                } else {
                    let _ = sqlx::query("UPDATE screenshots SET status = $1 WHERE id = $2")
                        .bind("failed")
                        .bind(screenshot_id)
                        .execute(&state.db)
                        .await;
                }
            }
            Err(_) => {
                let _ = sqlx::query("UPDATE screenshots SET status = $1 WHERE id = $2")
                    .bind("failed")
                    .bind(screenshot_id)
                    .execute(&state.db)
                    .await;
            }
        }
    }

    // Update test run status
    let _ = sqlx::query("UPDATE test_runs SET status = $1 WHERE id = $2")
        .bind("completed")
        .bind(test_run_id)
        .execute(&state.db)
        .await;
}
