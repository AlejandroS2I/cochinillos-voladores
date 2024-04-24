use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;

use crate::modelo::{ControladorModelo};
use crate::modelo::usuario::{Usuario, UsuarioCrear, ControladorUsuario};
use crate::Result;

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/usuarios", post(crear_usuario).get(listar_usuarios))
        .route("/usuarios/:id", delete(eliminar_usuario))
        .with_state(cm)
}

#[debug_handler]
async fn crear_usuario(
    State(cm): State<ControladorModelo>,
    Form(usuario_crear): Form<UsuarioCrear>
) -> Result<StatusCode> {
    println!("->> {:<12} - crear usuario", "CONTROLADOR");

    let usuario = ControladorUsuario::crear_usuario(cm, usuario_crear).await?;

    Ok(StatusCode::CREATED)
}

#[derive(Template)]
#[template(path = "elementos/listaUsuario.html")]
pub struct ListaUsuarioTemplate {
    usuarios: Vec<Usuario>
}

async fn listar_usuarios(
    State(cm): State<ControladorModelo>,
) -> Result<ListaUsuarioTemplate> {
    println!("->> {:<12} - listar usuarios", "CONTROLADOR");

    let usuarios = ControladorUsuario::listar_usuarios(cm).await?;

    Ok(ListaUsuarioTemplate { usuarios })
}

async fn eliminar_usuario(
    State(cm): State<ControladorModelo>,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    println!("->> {:<12} - listar usuarios", "CONTROLADOR");

    let usuario = ControladorUsuario::eliminar_usuario(cm, id).await?;

    Ok(StatusCode::OK)
}
