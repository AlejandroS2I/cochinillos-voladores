use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;

use crate::ctx::Ctx;
use crate::modelo::{ControladorModelo};
use crate::modelo::usuario::{Usuario, UsuarioCrear, ControladorUsuario};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/usuarios", post(crear_usuario).get(listar_usuarios))
        .route("/usuarios/:id", delete(eliminar_usuario))
        .with_state(cm)
}

#[debug_handler]
async fn crear_usuario(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(usuario_crear): Form<UsuarioCrear>
) -> Result<StatusCode> {
    println!("->> {:<12} - crear usuario", "CONTROLADOR");

    if !ctx.esAdministrador() {
        return Err(Error::SinPermisos);
    }

    let usuario = ControladorUsuario::crear_usuario(ctx, cm, usuario_crear).await?;

    Ok(StatusCode::CREATED)
}

#[derive(Template)]
#[template(path = "componentes/listaUsuario.html")]
pub struct ListaUsuarioTemplate {
    usuarios: Vec<Usuario>
}

async fn listar_usuarios(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
) -> Result<ListaUsuarioTemplate> {
    println!("->> {:<12} - listar usuarios", "CONTROLADOR");

    if !ctx.esAdministrador() {
        return Err(Error::SinPermisos);
    }

    let usuarios = ControladorUsuario::listar_usuarios(ctx, cm).await?;

    Ok(ListaUsuarioTemplate { usuarios })
}

async fn eliminar_usuario(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    println!("->> {:<12} - listar usuarios", "CONTROLADOR");

    if !ctx.esAdministrador() {
        return Err(Error::SinPermisos);
    }

    let usuario = ControladorUsuario::eliminar_usuario(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}