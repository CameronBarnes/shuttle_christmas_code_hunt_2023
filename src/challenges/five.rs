use axum::{extract::Query, routing::post, Json, Router};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default)]
struct Pagnation {
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn names(pagnation: Query<Pagnation>, Json(names): Json<Vec<String>>) -> String {
    if let Some(split) = pagnation.split {
        serde_json::to_string_pretty(
            &names
                .into_iter()
                .skip(pagnation.offset)
                .take(pagnation.limit.unwrap_or(usize::MAX))
                .chunks(split)
                .into_iter()
                .map(|chunk| chunk.collect_vec())
                .collect_vec(),
        )
        .unwrap()
    } else {
        serde_json::to_string_pretty(
            &names
                .into_iter()
                .skip(pagnation.offset)
                .take(pagnation.limit.unwrap_or(usize::MAX))
                .collect_vec(),
        )
        .unwrap()
    }
}

pub(super) fn get_router() -> Router {
    Router::new().route("/", post(names))
}
