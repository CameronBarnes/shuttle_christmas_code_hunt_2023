use std::collections::HashMap;

use axum::{
    http::{header::COOKIE, HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

async fn decode(headers: HeaderMap) -> Result<String, StatusCode> {
    String::from_utf8(
        BASE64_STANDARD
            .decode(
                headers
                    .get(COOKIE)
                    .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
                    .to_str()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .trim_start_matches("recipe="),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
struct Input {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

impl Input {
    fn separate(self) -> (HashMap<String, usize>, HashMap<String, usize>) {
        (self.recipe, self.pantry)
    }
}

#[derive(Serialize)]
struct Output {
    cookies: usize,
    pantry: HashMap<String, usize>,
}

async fn bake(headers: HeaderMap) -> Result<Json<Output>, StatusCode> {
    let json = String::from_utf8(
        BASE64_STANDARD
            .decode(
                headers
                    .get(COOKIE)
                    .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
                    .to_str()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .trim_start_matches("recipe="),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let input: Input =
        serde_json::from_str(&json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (recipe, mut pantry) = input.separate();
    let mut max = usize::MAX;
    for (name, amount) in &recipe {
        if *amount == 0 {
            continue;
        }
        max = max.min(
            pantry
                .get(name)
                .copied()
                .unwrap_or(0)
                .saturating_div(*amount),
        );
    }
    for (name, amount) in &recipe {
        if let Some(val) = pantry.get_mut(name) {
            *val -= amount * max;
        }
    }
    Ok(Json(Output {
        cookies: max,
        pantry,
    }))
}

pub(super) fn get_router() -> Router {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(bake))
}
