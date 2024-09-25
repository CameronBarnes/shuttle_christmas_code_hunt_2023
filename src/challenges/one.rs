use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn cube_bits(Path(num_str): Path<String>) -> Result<String, StatusCode> {
    num_str
        .split(['\\', '/'])
        .map(|str| str.parse::<i32>().unwrap())
        .reduce(|acc, e| acc ^ e)
        .map(|num| num.pow(3).to_string())
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub(super) fn get_router() -> Router {
    Router::new().route("/*nums", get(cube_bits))
}
