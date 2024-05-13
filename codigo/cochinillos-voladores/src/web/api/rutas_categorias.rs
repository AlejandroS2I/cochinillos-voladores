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
use crate::modelo::categorias::{ControladorCategoria, Categoria, CategoriaActualizar, CategoriaCrear};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/categorias", post(crear_categoria).put(actualizar_categoria))
        .route("/categorias/:id", delete(eliminar_categoria))
        .with_state(cm)
}

async fn crear_categoria(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(categoria_crear): Form<CategoriaCrear>
) -> Result<StatusCode> {
    let categoria = ControladorCategoria::crear_categoria(cm, categoria_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_categoria(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(categoria_actualizar): Form<CategoriaActualizar>
) -> Result<StatusCode> {
    let categoria = ControladorCategoria::actualizar_categoria(cm, categoria_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_categoria(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let categoria = ControladorCategoria::eliminar_categoria(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
