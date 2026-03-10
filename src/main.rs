use axum::{
    Router,
    routing::get,
    response::{Html, IntoResponse, Response},
    http::{StatusCode, header},
};
use tokio::{fs, time::{sleep, Duration}};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/sleep", get(slow))
        .route("/sw.js", get(service_worker))
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();

    info!("Listening on http://127.0.0.1:7878");

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    let hostname = hostname();
    match fs::read_to_string("hello.html").await {
        Ok(contents) => Html(contents.replace("{{hostname}}", &hostname)),
        Err(_) => Html(format!("<h1>Error: hello.html not found</h1>")),
    }
}

async fn slow() -> impl IntoResponse {
    sleep(Duration::from_secs(5)).await;
    let hostname = hostname();
    match fs::read_to_string("hello.html").await {
        Ok(contents) => Html(contents.replace("{{hostname}}", &hostname)),
        Err(_) => Html(format!("<h1>Error: hello.html not found</h1>")),
    }
}

async fn not_found() -> impl IntoResponse {
    let body = serve_html("404.html").await;
    (StatusCode::NOT_FOUND, body)
}

async fn service_worker() -> Response {
    match fs::read_to_string("sw.js").await {
        Ok(contents) => (
            [(header::CONTENT_TYPE, "application/javascript")],
            contents,
        ).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "sw.js not found").into_response(),
    }
}

async fn serve_html(filename: &str) -> Html<String> {
    match fs::read_to_string(filename).await {
        Ok(contents) => Html(contents),
        Err(_) => Html(format!("<h1>Error: {} not found</h1>", filename)),
    }
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}
