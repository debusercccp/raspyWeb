use axum::{
    Router,
    routing::get,
    response::{Html, IntoResponse, Response, Json},
    http::{StatusCode, header},
};
use tokio::{fs, time::{sleep, Duration}};
use tracing::info;
use serde::Serialize;

#[derive(Serialize)]
struct Stats {
    cpu: f32,
    ram: f32,
    ram_used_mb: u64,
    ram_total_mb: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/sleep", get(slow))
        .route("/sw.js", get(service_worker))
        .route("/stats", get(stats))
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
        Err(_) => Html("<h1>Error: hello.html not found</h1>".to_string()),
    }
}

async fn slow() -> impl IntoResponse {
    sleep(Duration::from_secs(5)).await;
    let hostname = hostname();
    match fs::read_to_string("hello.html").await {
        Ok(contents) => Html(contents.replace("{{hostname}}", &hostname)),
        Err(_) => Html("<h1>Error: hello.html not found</h1>".to_string()),
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

// GET /stats — returns real CPU and RAM usage as JSON
async fn stats() -> Json<Stats> {
    Json(Stats {
        cpu: read_cpu().await,
        ..read_ram()
    })
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

// Reads CPU usage by sampling /proc/stat twice 200ms apart
async fn read_cpu() -> f32 {
    fn parse_stat() -> Option<(u64, u64)> {
        let content = std::fs::read_to_string("/proc/stat").ok()?;
        let line = content.lines().next()?;
        let nums: Vec<u64> = line.split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() < 4 { return None; }
        let idle = nums[3];
        let total: u64 = nums.iter().sum();
        Some((idle, total))
    }

    let a = parse_stat();
    sleep(Duration::from_millis(200)).await;
    let b = parse_stat();

    match (a, b) {
        (Some((idle1, total1)), Some((idle2, total2))) => {
            let total_diff = total2 - total1;
            let idle_diff  = idle2  - idle1;
            if total_diff == 0 { return 0.0; }
            (1.0 - idle_diff as f32 / total_diff as f32) * 100.0
        }
        _ => 0.0,
    }
}

// Reads RAM usage from /proc/meminfo
fn read_ram() -> Stats {
    let content = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0u64;
    let mut available = 0u64;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        }
    }

    let used = total.saturating_sub(available);
    let pct = if total > 0 { used as f32 / total as f32 * 100.0 } else { 0.0 };

    Stats {
        cpu: 0.0,
        ram: pct,
        ram_used_mb: used / 1024,
        ram_total_mb: total / 1024,
    }
}
