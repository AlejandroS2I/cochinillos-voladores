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
use crate::modelo::tipos_evento::{ControladorTipoEvento, TipoEvento, TipoEventoActualizar, TipoEventoCrear};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/tiposEvento", post(crear_tipo_evento).put(actualizar_tipo_evento))
        .route("/tiposEvento/:id", delete(eliminar_tipo_evento))
        .with_state(cm)
}

async fn crear_tipo_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(tipo_evento_crear): Form<TipoEventoCrear>
) -> Result<StatusCode> {
    let tipo_evento = ControladorTipoEvento::crear_tipo_evento(cm, tipo_evento_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_tipo_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(tipo_evento_actualizar): Form<TipoEventoActualizar>
) -> Result<StatusCode> {
    let tipo_evento = ControladorTipoEvento::actualizar_tipo_evento(cm, tipo_evento_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_tipo_evento(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let tipo_evento = ControladorTipoEvento::eliminar_tipo_evento(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
