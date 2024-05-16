use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Form, Router};

use crate::ctx::Ctx;
use crate::modelo::ControladorModelo;
use crate::modelo::competiciones::{ControladorCompeticion, CompeticionActualizar, CompeticionCrear};
use crate::Result;

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/competiciones", post(crear_competicion).put(actualizar_competicion))
        .route("/competiciones/:id", delete(eliminar_competicion))
        .with_state(cm)
}

async fn crear_competicion(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(competicion_crear): Form<CompeticionCrear>
) -> Result<StatusCode> {
    let _competicion = ControladorCompeticion::crear_competicion(cm, competicion_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_competicion(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(competicion_actualizar): Form<CompeticionActualizar>
) -> Result<StatusCode> {
    let _competicion = ControladorCompeticion::actualizar_competicion(cm, competicion_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_competicion(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let _competicion = ControladorCompeticion::eliminar_competicion(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
