use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Form, Router};

use crate::ctx::Ctx;
use crate::modelo::ControladorModelo;
use crate::modelo::partidos::{ControladorPartido, PartidoActualizar, PartidoCrear};
use crate::Result;

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/partidos", post(crear_partido).put(actualizar_partido))
        .route("/partidos/:id", delete(eliminar_partido))
        .with_state(cm)
}

async fn crear_partido(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(partido_crear): Form<PartidoCrear>
) -> Result<StatusCode> {
    let _partido = ControladorPartido::crear_partido(cm, partido_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_partido(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(partido_actualizar): Form<PartidoActualizar>
) -> Result<StatusCode> {
    let _partido = ControladorPartido::actualizar_partido(cm, partido_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_partido(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let _partido = ControladorPartido::eliminar_partido(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
