use axum::{routing::post, Json, Router};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

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

pub(super) fn get_router() -> Router {
    Router::new()
        .route("/strength", post(reindeer_strength))
        .route("/contest", post(reindeer_contest))
}
