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
use crate::modelo::competiciones_equipos::{CompeticionEquipo, ControladorCompeticionEquipo};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/competicionesEquipos", post(crear_competicion_equipo).put(actualizar_competicion_equipo))
        .route("/competicionesEquipos/:idCompeticion/:idEquipo", delete(eliminar_competicion_equipo))
        .with_state(cm)
}

async fn crear_competicion_equipo(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(competicion_equipo): Form<CompeticionEquipo>
) -> Result<StatusCode> {
    let competicion_equipo = ControladorCompeticionEquipo::crear_competicion_equipo(cm, competicion_equipo).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_competicion_equipo(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Form(competicion_equipo): Form<CompeticionEquipo>
) -> Result<StatusCode> {
    let competicion_equipo = ControladorCompeticionEquipo::actualizar_competicion_equipo(cm, competicion_equipo).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_competicion_equipo(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path((idCompeticion, idEquipo)): Path<(u32, u32)>
) -> Result<StatusCode> {
    let competicion_equipo = ControladorCompeticionEquipo::eliminar_competicion_equipo(ctx, cm, idCompeticion, idEquipo).await?;

    Ok(StatusCode::OK)
}
