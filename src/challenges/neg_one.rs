use axum::{http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn always_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub(super) fn get_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(always_error))
}
