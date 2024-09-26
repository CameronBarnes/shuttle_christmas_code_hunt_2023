use axum::Router;

mod six;
mod five;
mod four;
mod neg_one;
mod one;

pub fn get_router() -> Router {
    Router::new()
        .nest("", neg_one::get_router())
        .nest("/1", one::get_router())
        .nest("/4", four::get_router())
        .nest("/5", five::get_router())
        .nest("/6", six::get_router())
}
