use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use image::GenericImageView;
use image_compare::Algorithm;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct DiffRequest {
    image1_url: String,
    image2_url: String,
}

#[derive(Serialize)]
struct DiffResponse {
    score: f64,
    is_different: bool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .route("/diff", post(compare_images))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 50052));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Diff Service listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn compare_images(Json(payload): Json<DiffRequest>) -> Json<DiffResponse> {
    let img1_bytes = reqwest::get(&payload.image1_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let img2_bytes = reqwest::get(&payload.image2_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let img1 = image::load_from_memory(&img1_bytes).unwrap().to_luma8();
    let img2 = image::load_from_memory(&img2_bytes).unwrap().to_luma8();

    // Basic comparison using image-compare (MSSIM)
    let similarity =
        image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &img1, &img2).unwrap();

    let score = similarity.score;

    // Score is similarity (0 to 1). 1.0 means identical.
    // We want to return if they are different.
    let is_different = score < 0.99;

    Json(DiffResponse {
        score,
        is_different,
    })
}
