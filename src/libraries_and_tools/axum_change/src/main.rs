use axum::extract::Path;
use axum::{Json, Router, routing::get};
use axum_change::greedy_coin_change;
use serde_json::json;

async fn root() -> &'static str {
    "
    Greedy Coin Change

    ** Primary Route: **
    /change/dollars/cents
    "
}

async fn change(Path((dollars, cents)): Path<(u32, u32)>) -> impl axum::response::IntoResponse {
    let amount = dollars * 100 + cents;
    let change = greedy_coin_change(amount);
    let json = json!({
        "dollars": dollars,
        "cents": cents,
        "change": change
    });
    Json(json)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/change/:dollars/:cents", get(change));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
