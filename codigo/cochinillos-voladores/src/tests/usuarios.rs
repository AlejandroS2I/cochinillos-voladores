pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use axum::{
    body::Body,
    http::{Request, StatusCode}
};
use tower::util::ServiceExt;
use sqlx::{MySqlPool};

use crate::modelo::{ControladorModelo, usuario::ControladorUsuario};
use crate::app;

#[sqlx::test(fixtures("usuarios"))]
async fn obtener_usuario(pool: MySqlPool) -> Result<()> {
    let cm = ControladorModelo::new(pool).await?;

    let usuario = ControladorUsuario::usuario_id(cm, 1).await
        .unwrap_or_else(|err| panic!("No se ha conseguido el usuario 1"))
        .ok_or_else(|| panic!("No se ha conseguido el usuario 1")).unwrap();

    assert_eq!(usuario.id, 1);

    Ok(())
}
