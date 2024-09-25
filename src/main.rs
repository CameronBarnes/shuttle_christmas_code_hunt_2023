use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::info;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn always_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn cube_bits(Path(num_str): Path<String>) -> Result<String, StatusCode> {
    info!(num_str);
    num_str
        .split(['\\', '/'])
        .map(|str| str.parse::<i32>().unwrap())
        .reduce(|acc, e| acc ^ e)
        .map(|num| num.pow(3).to_string())
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
struct Reindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

impl Reindeer {
    fn strength(&self) -> i32 {
        self.strength
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn antler_width(&self) -> i32 {
        self.antler_width
    }

    fn snow_magic_power(&self) -> i32 {
        self.snow_magic_power
    }

    fn favorite_food(&self) -> &str {
        &self.favorite_food
    }

    fn candies_eaten_yesterday(&self) -> i32 {
        self.candies_eaten_yesterday
    }
}

#[derive(Serialize)]
struct Results {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn reindeer_strength(Json(deer): Json<Vec<Reindeer>>) -> String {
    deer.iter().map(Reindeer::strength).sum::<i32>().to_string()
}

async fn reindeer_contest(Json(deer): Json<Vec<Reindeer>>) -> Json<Results> {
    let fastest = deer
        .iter()
        .sorted_by(|a, b| a.speed().partial_cmp(&b.speed()).unwrap())
        .last()
        .unwrap();
    let fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength(),
        fastest.name()
    );
    let tallest = deer.iter().sorted_by_key(|a| a.height()).last().unwrap();
    let tallest = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest.name(),
        tallest.antler_width()
    );
    let magician = deer
        .iter()
        .sorted_by_key(|a| a.snow_magic_power())
        .last()
        .unwrap();
    let magician = format!(
        "{} could blast you away with a snow magic power of {}",
        magician.name(),
        magician.snow_magic_power()
    );
    let consumer = deer
        .iter()
        .sorted_by_key(|a| a.candies_eaten_yesterday())
        .last()
        .unwrap();
    let consumer = format!(
        "{} ate lots of candies, but also some {}",
        consumer.name(),
        consumer.favorite_food()
    );
    Json(Results {
        fastest,
        tallest,
        magician,
        consumer,
    })
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(always_error))
        .route("/1/*nums", get(cube_bits))
        .route("/4/strength", post(reindeer_strength))
        .route("/4/contest", post(reindeer_contest));

    Ok(router.into())
}
