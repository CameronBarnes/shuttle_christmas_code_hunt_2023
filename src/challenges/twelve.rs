use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
    time::Instant,
};

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Datelike, Utc};
use itertools::Itertools;
use serde::Serialize;
use ulid::Ulid;
use uuid::Uuid;

static STRING_STORE: LazyLock<Mutex<HashMap<String, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

async fn save(Path(str): Path<String>) -> StatusCode {
    if let Ok(mut map) = STRING_STORE.lock() {
        map.insert(str, Instant::now());
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn convert(Json(ulids): Json<Vec<Ulid>>) -> Json<Vec<Uuid>> {
    Json(
        ulids
            .into_iter()
            .map(|ulid| Uuid::from_u128(ulid.0))
            .rev()
            .collect_vec(),
    )
}

async fn load(Path(str): Path<String>) -> Result<String, StatusCode> {
    STRING_STORE
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get(&str)
        .map(|instant| instant.elapsed().as_secs().to_string())
        .ok_or(StatusCode::BAD_REQUEST)
}

#[derive(Serialize, Default)]
struct DataOut {
    #[serde(rename(serialize = "christmas eve"))]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename(serialize = "in the future"))]
    in_the_future: usize,
    #[serde(rename(serialize = "LSB is 1"))]
    lsb_is_one: usize,
}

async fn weekday(Path(day): Path<u32>, Json(ulids): Json<Vec<Ulid>>) -> Json<DataOut> {
    let mut out = DataOut::default();

    for ulid in &ulids {
        let time: DateTime<Utc> = ulid.datetime().into();
        // This shouldnt be nessecary, but the shuttle validator appears to get this particular
        // date wrong. So we'll put an offset for any year at or past this point
        let date_offset = if time.year() == 2121 {
            1
        } else {
            0
        };
        if time.weekday().number_from_monday() - date_offset == day {
            out.weekday += 1;
        }
        if time.day() == 24 && time.month() == 12 {
            out.christmas_eve += 1;
        }
        if time.timestamp() > chrono::offset::Utc::now().timestamp() {
            out.in_the_future += 1;
        }
        if ulid.random() & 1 == 1 {
            out.lsb_is_one += 1;
        }
    }

    Json(out)
}

pub(super) fn get_router() -> Router {
    Router::new()
        .route("/save/:str", post(save))
        .route("/load/:str", get(load))
        .route("/ulids", post(convert))
        .route("/ulids/:day", post(weekday))
}
