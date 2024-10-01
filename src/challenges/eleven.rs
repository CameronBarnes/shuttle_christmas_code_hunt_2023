use std::io::Cursor;

use axum::{extract::Multipart, http::StatusCode, routing::post, Router};
use image::{GenericImageView, ImageReader, Pixel};
use tower_http::services::ServeDir;

async fn magic_red(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name.eq_ignore_ascii_case("image") {
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            let img = ImageReader::new(Cursor::new(data))
                .with_guessed_format()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .decode()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let red = img
                .pixels()
                .filter(|(_, _, rgb)| {
                    let channels = rgb.channels();
                    channels[0] as u16 > (channels[1] as u16 + channels[2] as u16)
                })
                .count();
            return Ok(red.to_string());
        }
    }
    Ok(String::from("0"))
}

pub(super) fn get_router() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/red_pixels", post(magic_red))
}
