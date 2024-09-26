use std::sync::LazyLock;

use axum::{routing::post, Json, Router};
use regex::Regex;
use serde::Serialize;

fn get_elves(input: &str) -> usize {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("elf").unwrap());
    RE.captures_iter(input).count()
}

fn get_elves_on_shelves(input: &str) -> usize {
    let mut count = 0;
    let mut str = input.to_string();
    while str.contains("elf on a shelf") {
        count += 1;
        str = str.replacen(" on a shelf", "", 1);
    }
    count
}

fn get_shelves_without_elves(input: &str) -> usize {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("shelf").unwrap());
    let mut str = input.to_string();
    while str.contains("elf on a shelf") {
        str = str.replacen(" on a shelf", "", 1);
    }
    RE.captures_iter(&str).count()
}

#[derive(Serialize)]
struct Elves {
    elf: usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_without_elf: usize,
}

async fn elves(body: String) -> Json<Elves> {
    Json(Elves {
        elf: get_elves(&body),
        elf_on_shelf: get_elves_on_shelves(&body),
        shelf_without_elf: get_shelves_without_elves(&body),
    })
}

pub(super) fn get_router() -> axum::Router {
    Router::new().route("/", post(elves))
}
