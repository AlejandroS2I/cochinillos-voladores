use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};
use axum::response::{IntoResponse, Response};
use axum::debug_handler;
use askama::Template;
use serde::Deserialize;

use crate::ctx::Ctx;
use crate::modelo::pwd::verificar_password;
use crate::modelo::{ControladorModelo};
use crate::modelo::usuario::{ControladorUsuario, Usuario, UsuarioActualizar, UsuarioCrear, UsuarioPassword};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/usuarios", post(crear_usuario).get(listar_usuarios).put(actualizar_usuario))
        .route("/usuarios/:id", delete(eliminar_usuario))
        .route("/cambiarPassword", put(cambiar_password))
        .with_state(cm)
}

async fn crear_usuario(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(usuario_crear): Form<UsuarioCrear>
) -> Result<StatusCode> {
    if !ctx.usuario().esAdministrador {
        return Err(Error::SinPermisos);
    }

    let usuario = ControladorUsuario::crear_usuario(cm, usuario_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_usuario(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(usuario_actualizar): Form<UsuarioActualizar>
) -> Result<StatusCode> {
    if !ctx.usuario().esAdministrador && ctx.usuario().id != usuario_actualizar.id {
        return Err(Error::SinPermisos);
    }

    let usuario = ControladorUsuario::actualizar_usuario(cm, usuario_actualizar).await?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct PasswordPayload {
    id: u32,
    passwordActual: String,
    passwordNueva: String,
    passwordRepetir: String
}

async fn cambiar_password(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(payload): Form<PasswordPayload>
) -> Result<StatusCode> {
    if ctx.usuario().id != payload.id {
        return Err(Error::SinPermisos);
    }

    let usuario = ControladorUsuario::usuario_id(cm.clone(), payload.id).await?
        .ok_or(Error::ErrorLoginMailNoEncontrado)?;

    if payload.passwordNueva != payload.passwordRepetir {
        return Err(Error::PasswordNoCoinciden);
    };

    if !verificar_password(payload.passwordActual.clone(), &usuario.password)? {
        return Err(Error::PasswordIncorrecta);
    };

    let usuario_password = UsuarioPassword {
        id: payload.id,
        password: payload.passwordNueva
    };

    ControladorUsuario::cambiar_password(cm, usuario_password).await?;

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
    if !ctx.usuario().esAdministrador {
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
    if !ctx.usuario().esAdministrador {
        return Err(Error::SinPermisos);
    }

    let usuario = ControladorUsuario::eliminar_usuario(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
