pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use axum::{
    body::Body,
    http::{Request, StatusCode}
};
use dotenvy::dotenv;
use tower::util::ServiceExt;
use sqlx::{MySqlPool};

use crate::modelo::ControladorModelo;
use crate::app;

mod usuarios;

#[sqlx::test]
async fn inicio(pool: MySqlPool) -> Result<()> {
    let cm = ControladorModelo::new(pool).await?;

    dotenv().ok();
    let ruta_uploads = std::env::var("RUTA_UPLOADS").expect("ERROR: Ruta uploads no especificada");

    let app = app(cm, ruta_uploads);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
