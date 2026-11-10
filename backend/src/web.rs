use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../frontend/build/"]
struct Assets;

pub fn router() -> Router {
    Router::new().fallback(static_handler)
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(&path) {
        Some(file) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap())
                .body(Body::from(file.data))
                .unwrap()
        }
        None => {
            // SPA-Fallback: Falls Pfad nicht existiert (z. B. Client-Side Routing), index.html liefern
            match Assets::get("index.html") {
                Some(file) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"))
                    .body(Body::from(file.data))
                    .unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Frontend build not found"))
                    .unwrap(),
            }
        }
    }
}
