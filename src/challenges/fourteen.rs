use axum::{routing::post, Json, Router};
use serde::Deserialize;

static BASE: &str = r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {replace}
  </body>
</html>"#;

#[derive(Deserialize)]
struct Content {
    content: String,
}

async fn not_safe(Json(content): Json<Content>) -> String {
    BASE.replace("{replace}", &content.content)
}

// This is escaping correctly, but the result it's expecting doesnt escape the '/'
async fn safe(Json(content): Json<Content>) -> String {
    BASE.replace(
        "{replace}",
        &html_escape::encode_safe(&content.content).replace("&#x2F;", "/"),
    )
}

pub(super) fn get_router() -> Router {
    Router::new()
        .route("/unsafe", post(not_safe))
        .route("/safe", post(safe))
}
