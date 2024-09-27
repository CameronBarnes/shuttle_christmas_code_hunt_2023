use std::{ops::Div, sync::LazyLock};

use axum::{extract::Path, http::StatusCode, routing::get, Router};
use rustemon::{client::RustemonClient, model::pokemon::Pokemon, pokemon::pokemon};

async fn get_pokemon(id: i64) -> Result<Pokemon, StatusCode> {
    static CLIENT: LazyLock<RustemonClient> = LazyLock::new(RustemonClient::default);
    pokemon::get_by_id(id, &CLIENT)
        .await
        .map_err(|_| StatusCode::FAILED_DEPENDENCY)
}

async fn weight(Path(id): Path<i64>) -> Result<String, StatusCode> {
    Ok((get_pokemon(id).await?.weight as f64).div(10.0).to_string())
}

async fn drop(Path(id): Path<i64>) -> Result<String, StatusCode> {
    let velocity = (2.0 * 9.825_f64 * 10.0).sqrt();
    let pokemon_mass = (get_pokemon(id).await?.weight as f64).div(10.0);
    let momentum = velocity * pokemon_mass;
    Ok(momentum.to_string())
}

pub(super) fn get_router() -> Router {
    Router::new()
        .route("/weight/:id", get(weight))
        .route("/drop/:id", get(drop))
}
