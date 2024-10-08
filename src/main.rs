use axum::Router;
use sqlx::PgPool;

mod challenges;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest("", challenges::get_router(pool));

    Ok(router.into())
}
