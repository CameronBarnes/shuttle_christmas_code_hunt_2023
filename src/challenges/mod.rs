use axum::Router;

mod eleven;
mod eight;
mod five;
mod four;
mod neg_one;
mod one;
mod seven;
mod six;

pub fn get_router() -> Router {
    Router::new()
        .nest("", neg_one::get_router())
        .nest("/1", one::get_router())
        .nest("/4", four::get_router())
        .nest("/5", five::get_router())
        .nest("/6", six::get_router())
        .nest("/7", seven::get_router())
        .nest("/8", eight::get_router())
        .nest("/11", eleven::get_router())
}
