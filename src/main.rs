use axum::Router;

mod challenges;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest("", challenges::get_router());

    Ok(router.into())
}
