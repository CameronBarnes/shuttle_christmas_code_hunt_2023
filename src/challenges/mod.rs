use axum::Router;
use sqlx::PgPool;

mod eight;
mod eleven;
mod five;
mod four;
mod neg_one;
mod one;
mod seven;
mod six;
mod thirteen;
mod twelve;

pub fn get_router(pool: PgPool) -> Router {
    Router::new()
        .nest("", neg_one::get_router())
        .nest("/1", one::get_router())
        .nest("/4", four::get_router())
        .nest("/5", five::get_router())
        .nest("/6", six::get_router())
        .nest("/7", seven::get_router())
        .nest("/8", eight::get_router())
        .nest("/11", eleven::get_router())
        .nest("/12", twelve::get_router())
        .nest("/13", thirteen::get_router(pool))
}
