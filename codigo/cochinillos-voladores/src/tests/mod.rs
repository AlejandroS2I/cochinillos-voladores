pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use axum::{
    body::Body,
    http::{Request, StatusCode}
};
use tower::util::ServiceExt;
use sqlx::{MySqlPool};

use crate::modelo::ControladorModelo;
use crate::app;

mod usuarios;

#[sqlx::test]
async fn inicio(pool: MySqlPool) -> Result<()> {
    let cm = ControladorModelo::new(pool).await?;

    let app = app(cm);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
