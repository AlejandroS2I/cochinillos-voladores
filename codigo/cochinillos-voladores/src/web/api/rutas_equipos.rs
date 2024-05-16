use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};

use crate::ctx::Ctx;
use crate::modelo::{ControladorModelo};
use crate::modelo::equipos::{ControladorEquipo, EquipoActualizar, EquipoCrear};
use crate::{Result, Error};

pub fn routes(cm: ControladorModelo) -> Router {
    Router::new()
        .route("/equipos", post(crear_equipo).put(actualizar_equipo))
        .route("/equipos/:id", delete(eliminar_equipo))
        .with_state(cm)
}

async fn crear_equipo(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(equipo_crear): Form<EquipoCrear>
) -> Result<StatusCode> {
    let _equipo = ControladorEquipo::crear_equipo(cm, equipo_crear).await?;

    Ok(StatusCode::CREATED)
}

async fn actualizar_equipo(
    State(cm): State<ControladorModelo>,
    _ctx: Ctx,
    Form(equipo_actualizar): Form<EquipoActualizar>
) -> Result<StatusCode> {
    let _equipo = ControladorEquipo::actualizar_equipo(cm, equipo_actualizar).await?;

    Ok(StatusCode::CREATED)
}

async fn eliminar_equipo(
    State(cm): State<ControladorModelo>,
    ctx: Ctx,
    Path(id): Path<u32>
) -> Result<StatusCode> {
    let _equipo = ControladorEquipo::eliminar_equipo(ctx, cm, id).await?;

    Ok(StatusCode::OK)
}
