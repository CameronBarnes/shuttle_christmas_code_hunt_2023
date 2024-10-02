use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, PgPool, Postgres, QueryBuilder};

static SCHEMA: &str = r#"DROP TABLE IF EXISTS orders;
CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
);"#;

#[derive(Deserialize, FromRow)]
struct RowInt(i32);

async fn sql(State(pool): State<PgPool>) -> Result<String, StatusCode> {
    let row: RowInt = sqlx::query_as("SELECT 20231213 number")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(row.0.to_string())
}

async fn reset(State(pool): State<PgPool>) -> StatusCode {
    match pool.execute(SCHEMA).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Serialize, Deserialize, FromRow)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[derive(Serialize, Default, FromRow)]
struct TotalResult {
    total: i64,
}

async fn orders(State(pool): State<PgPool>, Json(orders): Json<Vec<Order>>) -> StatusCode {
    if orders.is_empty() {
        return StatusCode::OK;
    }
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO orders (id, region_id, gift_name, quantity)");
    query_builder.push_values(orders, |mut b, order| {
        b.push_bind(order.id)
            .push_bind(order.region_id)
            .push_bind(order.gift_name)
            .push_bind(order.quantity);
    });
    match query_builder.build().execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Serialize, FromRow, Default)]
struct PopularResult {
    popular: Option<String>,
}

async fn total(State(pool): State<PgPool>) -> Result<Json<TotalResult>, StatusCode> {
    match sqlx::query_as("SELECT SUM(quantity) AS total FROM orders")
        .fetch_one(&pool)
        .await
    {
        Ok(total) => Ok(Json(total)),
        Err(e) => {
            tracing::error!("Error getting total: {}", e.to_string());
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn popular(State(pool): State<PgPool>) -> Result<Json<PopularResult>, StatusCode> {
    match sqlx::query_as("SELECT gift_name AS popular FROM orders GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT 1;").fetch_optional(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        Some(popular) => Ok(Json(popular)),
        None => Ok(Json(PopularResult::default()))
    }
}

pub(super) fn get_router(pool: PgPool) -> Router {
    Router::new()
        .route("/sql", get(sql))
        .route("/reset", post(reset))
        .route("/orders", post(orders))
        .route("/orders/total", get(total))
        .route("/orders/popular", get(popular))
        .with_state(pool)
}
