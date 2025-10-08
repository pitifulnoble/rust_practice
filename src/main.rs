use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/echo/:name", get(echo_path))
        .route("/sum", get(sum_query))
        .route("/echo_json", post(echo_json))
        .route("/api/users/login", post(users_login))
        ;

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 3001))
        .await
        .expect("failed to bind 127.0.0.1:3001");
    axum::serve(listener, app).await.expect("server error");
}

async fn hello() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "hello from rust-fist"
    }))
}

async fn echo_path(Path(_name): Path<String>) -> impl IntoResponse {
    // Accepts a path parameter but returns a fixed result
    Json(serde_json::json!({
        "result": "fixed-response"
    }))
}

#[derive(Deserialize)]
struct SumParams {
    x: i64,
    y: i64,
}

async fn sum_query(Query(_p): Query<SumParams>) -> impl IntoResponse {
    // Accepts query params but returns a fixed result
    Json(serde_json::json!({
        "sum": 42
    }))
}

#[derive(Deserialize)]
struct EchoInput {
    any: String,
}

#[derive(Serialize)]
struct EchoOutput {
    status: &'static str,
}

async fn echo_json(Json(_body): Json<EchoInput>) -> impl IntoResponse {
    // Accepts JSON body but returns a fixed result
    Json(EchoOutput { status: "accepted" })
}

async fn users_login() -> impl IntoResponse {
    Json(serde_json::json!({
        "code": 0,
        "message": "hello from rust-fist",
        "data" : "user_login_token_by_rust"
    }))
}

struct WorkFlow {
    work_flow: String,
    node_list: Vec<Node>,
}
struct Node {
    node_name: String,
    node_execute_service:String,
    node_param:String,
    next_node_name:String
}